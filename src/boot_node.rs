use crate::handlers::boot::handle_join_request::handle_join_request;
use crate::handlers::boot::handle_leave_request::handle_leave_request;
use crate::handlers::boot::handle_task_completed::handle_task_completed;
use crate::handlers::boot::set_expected_counter::set_expected_counter;
use crate::node::SyncResolve;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use crate::utils::{draw_voronoi_full, Voronoi};
use rand::Rng;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use zenoh::prelude::r#async::AsyncResolve;
pub use zenoh::prelude::sync::*;

/// A boot node in a network acts as the entry point for new nodes to join the cluster. It also acts as a central point for nodes to leave the cluster. Stores the site and polygon information of all nodes in the network. Constructs the distributed voronoi diagram from received polygons as well as a centralized voronoi diagram.
pub struct BootNode {
    pub(crate) session: Arc<Session>,
    pub(crate) zid: String,
    pub(crate) cluster_name: String,
    pub(crate) api_requester_tx: flume::Sender<BootApiMessage>,
    pub(crate) api_responder_rx: flume::Receiver<BootApiResponse>,
}

#[derive(Clone)]
pub struct BootNodeData {
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub cluster: OrderedMapPairs,
    pub polygon_list: OrderedMapPolygon,
    pub correct_polygon_list: OrderedMapPolygon,
    pub draw_count: i32,
    pub(crate) centralized_voronoi: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BootApiMessage {
    GetDrawCount,
    GetPolygonList,
    GetCorrectPolygonList,
    GetCluster,
}
#[derive(PartialEq, Clone, Debug)]
pub enum BootApiResponse {
    GetDrawCount(i32),
    GetPolygonList(Vec<(String, Vec<(f64, f64)>)>),
    GetCorrectPolygonList(Vec<(String, Vec<(f64, f64)>)>),
    GetCluster(Vec<(String, (f64, f64))>),
}

impl BootNode {
    pub fn new(cluster_name: &str, centralized_voronoi: bool) -> Self {
        let session = zenoh::open(Config::default())
            .res_sync()
            .unwrap()
            .into_arc();
        let zid = session.zid().to_string();
        let cluster_name_clone = cluster_name.to_string();
        let session_clone = Arc::clone(&session);
        let (tx, rx) = flume::unbounded();
        let (tx2, rx2) = flume::unbounded();
        async_std::task::spawn(async move {
            let boot_subscriber = session_clone
                .declare_subscriber(format!("{}/node/boot/*", cluster_name_clone))
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            let counter_subscriber = session_clone
                .declare_subscriber(format!("{}/counter/*", cluster_name_clone))
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            loop {
                futures::select! {
                       sample = boot_subscriber.recv_async() => {
                        if let Ok(sample) = sample {
                            tx.send(sample).unwrap();
                        } else {
                            break; // Exit the loop if receiving from `node_update_rx` fails
                        }
                    }
                    sample = counter_subscriber.recv_async() => {
                        if let Ok(sample) = sample {
                             tx2.send(sample).unwrap();
                        } else {
                            break; // Exit the loop if receiving from `api_requester_rx` fails
                        }
                    }
                }
            }
        });

        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = cluster_name.to_string();
        let session_clone = Arc::clone(&session);
        let (api_requester_tx, api_requester_rx) = flume::unbounded();
        let (api_responder_tx, api_responder_rx) = flume::unbounded();
        async_std::task::spawn_blocking(move || {
            let mut boot_node_data = BootNodeData::new(centralized_voronoi);
            loop {
                if let Ok(sample) = rx.try_recv() {
                    boot_node_data.expected_counter = -1;
                    boot_node_data.received_counter = 0;
                    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                    println!("Message received on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    let session_clone = session_clone.clone();
                    let cluster_name = cluster_name_clone.as_str();
                    match topic {
                        "new" => {
                            handle_join_request(
                                payload,
                                &mut boot_node_data,
                                session_clone,
                                cluster_name,
                                zid_clone.as_str(),
                            );
                        }
                        "leave_request" => {
                            handle_leave_request(
                                payload,
                                &mut boot_node_data,
                                session_clone,
                                cluster_name,
                            );
                        }
                        _ => println!("UNKNOWN BOOT TOPIC"),
                    }
                    while boot_node_data.expected_counter != boot_node_data.received_counter {
                        while let Ok(sample) = rx2.try_recv() {
                            let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
                            println!("Message received on topic... {:?}", topic);
                            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                            match topic {
                                "expected_wait" => {
                                    set_expected_counter(
                                        payload,
                                        &mut boot_node_data.expected_counter,
                                    );
                                }
                                "complete" => {
                                    handle_task_completed(payload, &mut boot_node_data);
                                }
                                _ => println!("UNKNOWN COUNTER TOPIC"),
                            }
                        }
                    }
                    if !boot_node_data.cluster.is_empty() {
                        //redraw distributed voronoi
                        draw_voronoi_full(
                            &boot_node_data.cluster,
                            &boot_node_data.polygon_list,
                            format!("{}voronoi", boot_node_data.draw_count).as_str(),
                        );
                        if boot_node_data.centralized_voronoi {
                            //correct voronoi
                            let mut hash_map = boot_node_data.cluster.clone();
                            boot_node_data.correct_polygon_list = OrderedMapPolygon::new();
                            let (first_zid, first_site) = hash_map
                                .iter()
                                .next()
                                .map(|(k, v)| (k.clone(), *v))
                                .unwrap();
                            hash_map.swap_remove_index(0);
                            let diagram = Voronoi::new((first_zid, first_site), &hash_map);
                            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                                let site_id = diagram.input.keys().nth(i).unwrap();
                                boot_node_data
                                    .correct_polygon_list
                                    .insert(site_id.to_string(), polygon);
                            }
                            draw_voronoi_full(
                                &boot_node_data.cluster,
                                &boot_node_data.correct_polygon_list,
                                format!("{}confirm", boot_node_data.draw_count).as_str(),
                            );
                        }
                        boot_node_data.draw_count += 1;
                    };
                }
                if let Ok(api_message) = api_requester_rx.try_recv() {
                    let api_response = match api_message {
                        BootApiMessage::GetCluster => BootApiResponse::GetCluster(
                            boot_node_data.cluster.clone().into_iter().collect(),
                        ),
                        BootApiMessage::GetPolygonList => BootApiResponse::GetPolygonList(
                            boot_node_data
                                .polygon_list
                                .clone()
                                .into_iter()
                                .collect(),
                        ),
                        BootApiMessage::GetCorrectPolygonList => {
                            BootApiResponse::GetCorrectPolygonList(
                                boot_node_data
                                    .correct_polygon_list
                                    .clone()
                                    .into_iter()
                                    .collect(),
                            )
                        }
                        BootApiMessage::GetDrawCount => {
                            BootApiResponse::GetDrawCount(boot_node_data.draw_count)
                        }
                    };
                    api_responder_tx.send(api_response).unwrap();
                }
            }
        });

        Self {
            session,
            zid,
            cluster_name: cluster_name.to_string(),
            api_requester_tx,
            api_responder_rx,
        }
    }

    /// Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::boot).
    pub fn start(&mut self) {}

    /// Get the zid of the node
    pub fn get_zid(&self) -> String {
        self.zid.clone()
    }

    pub fn get_cluster(&self) -> Vec<(String, (f64, f64))> {
        self.api_requester_tx
            .send(BootApiMessage::GetCluster)
            .unwrap();
        if let BootApiResponse::GetCluster(cluster) = self.api_responder_rx.recv().unwrap() {
            cluster
        } else {
            panic!("Wrong response type");
        }
    }

    pub fn get_polygon_list(&self) -> Vec<(String, Vec<(f64, f64)>)> {
        self.api_requester_tx
            .send(BootApiMessage::GetPolygonList)
            .unwrap();
        if let BootApiResponse::GetPolygonList(polygon_list) = self.api_responder_rx.recv().unwrap()
        {
            polygon_list
        } else {
            panic!("Wrong response type");
        }
    }
    pub fn get_correct_polygon_list(&self) -> Vec<(String, Vec<(f64, f64)>)> {
        self.api_requester_tx
            .send(BootApiMessage::GetCorrectPolygonList)
            .unwrap();
        if let BootApiResponse::GetCorrectPolygonList(correct_polygon_list) =
            self.api_responder_rx.recv().unwrap()
        {
            correct_polygon_list
        } else {
            panic!("Wrong response type");
        }
    }
    pub fn get_draw_count(&self) -> i32 {
        self.api_requester_tx
            .send(BootApiMessage::GetDrawCount)
            .unwrap();
        if let BootApiResponse::GetDrawCount(draw_count) = self.api_responder_rx.recv().unwrap() {
            draw_count
        } else {
            panic!("Wrong response type");
        }
    }
}

impl BootNodeData {
    /// Creates a new boot node instance with a [session](https://docs.rs/zenoh/0.7.0-rc/zenoh/struct.Session.html).
    /// Opens a subscription on topic `{cluster}/boot/*` to receive incoming messages from nodes and a subscription on`{cluster}/counter/*` to count the number of messages its received since processing the current message on the `{cluster}/boot/*` topic.
    pub fn new(centralized_voronoi: bool) -> Self {
        Self {
            received_counter: 0,
            expected_counter: -1,
            cluster: OrderedMapPairs::new(),
            polygon_list: OrderedMapPolygon::new(),
            correct_polygon_list: OrderedMapPolygon::new(),
            draw_count: 0,
            centralized_voronoi,
        }
    }
}

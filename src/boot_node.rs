use crate::handlers::boot::boot_api_matcher::{boot_api_matcher, BootApiMessage, BootApiResponse};
use crate::handlers::boot::boot_topic_matcher::{boot_counter_topic_matcher, boot_topic_matcher};
use crate::handlers::boot::generate_output::generate_output;
use crate::node::SyncResolve;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use std::sync::Arc;
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
                            break;
                        }
                    }
                    sample = counter_subscriber.recv_async() => {
                        if let Ok(sample) = sample {
                             tx2.send(sample).unwrap();
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = session.zid().to_string();
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

                    boot_topic_matcher(
                        topic,
                        payload,
                        &mut boot_node_data,
                        session_clone.clone(),
                        zid_clone.as_str(),
                        cluster_name_clone.as_str(),
                    );

                    while boot_node_data.expected_counter != boot_node_data.received_counter {
                        while let Ok(sample) = rx2.try_recv() {
                            let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
                            println!("Message received on topic... {:?}", topic);
                            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();

                            boot_counter_topic_matcher(topic, payload, &mut boot_node_data);
                        }
                    }
                    if !boot_node_data.cluster.is_empty() {
                        generate_output(&mut boot_node_data)
                    };
                }
                if let Ok(api_message) = api_requester_rx.try_recv() {
                    boot_api_matcher(api_message, &mut boot_node_data, &api_responder_tx);
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

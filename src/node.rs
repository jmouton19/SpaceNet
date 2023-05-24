use crate::handlers::node::handle_leave_response::handle_leave_response;
use crate::handlers::node::handle_leave_voronoi_request::handle_leave_voronoi_request;
use crate::handlers::node::handle_neighbours_neighbours_request::handle_neighbours_neighbours_request;
use crate::handlers::node::handle_neighbours_neighbours_response::handle_neighbours_neighbours_response;
use crate::handlers::node::handle_new_voronoi_request::handle_new_voronoi_request;
use crate::handlers::node::handle_owner_request::handle_owner_request;
use crate::handlers::node::handle_owner_response::handle_owner_response;
use crate::message::DefaultMessage;
use crate::types::OrderedMapPairs;
use async_std::io::ReadExt;
use async_std::{io, task};
use bincode::serialize;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rand::Rng;

use zenoh::prelude::r#async::AsyncResolve;

pub use zenoh::prelude::sync::*;

/// A node in a network that has a point site which is used in the calculation of the voronoi diagram of a cluster. Computes its own voronoi polygon from its list of neighbours. Does not store information on entire cluster.
#[derive(Clone)]
pub struct NodeData {
    pub(crate) site: (f64, f64),
    pub(crate) neighbours: OrderedMapPairs,
    pub(crate) k_hop_neighbours: OrderedMapPairs,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub(crate) polygon: Vec<(f64, f64)>,
    pub(crate) status: NodeStatus,
}

pub struct Node {
    pub(crate) session: Arc<Session>,
    pub(crate) cluster_name: String,
    pub(crate) zid: String,
    pub(crate) api_requester_tx: flume::Sender<ApiMessage>,
    pub(crate) api_responder_rx: flume::Receiver<ApiResponse>,
    // subscription: Option<Subscriber<'a, ()>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ApiMessage {
    GetStatus,
    GetNeighbours,
    GetPolygon,
    IsNeighbour(String),
}
#[derive(PartialEq, Clone, Debug)]
pub enum ApiResponse {
    GetStatusResponse(NodeStatus),
    GetNeighboursResponse(Vec<(String, (f64, f64))>),
    GetPolygonResponse(Vec<(f64, f64)>),
    IsNeighbourResponse(bool),
}

impl Node {
    pub fn new(cluster_name: &str) -> Self {
        let session = zenoh::open(config::default())
            .res_sync()
            .unwrap()
            .into_arc();
        let zid = session.zid().to_string();

        let session_clone = Arc::clone(&session);
        let (zenoh_tx, zenoh_rx) = flume::unbounded();
        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = zid.to_string();
        //task to listen for zenoh messages, sends message to processor task.
        async_std::task::spawn(async move {
            let subscriber = session_clone
                .declare_subscriber(format!("{}/node/{}/*", cluster_name_clone, zid_clone))
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            while let Ok(sample) = subscriber.recv_async().await {
                zenoh_tx.send(sample).unwrap();
            }
        });

        //Processor task, handles messages from zenoh. Update node_data copy in API task
        let session_clone = Arc::clone(&session);
        let (node_update_tx, node_update_rx) = flume::unbounded();
        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = zid.to_string();

        async_std::task::spawn_blocking(move || {
            let mut node_data = NodeData::new();
            loop {
                while let Ok(sample) = zenoh_rx.try_recv() {
                    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                    println!("Received message on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    let session_clone = session_clone.clone();
                    let cluster_name = cluster_name_clone.as_str();
                    let zid = zid_clone.as_str();
                    // let mut rng = rand::thread_rng();
                    // let delay = rng.gen_range(1..=10);
                    // thread::sleep(Duration::from_millis(delay));
                    match topic {
                        "new_reply" => {
                            handle_owner_request(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "owner_request" => {
                            handle_owner_request(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "owner_response" => {
                            handle_owner_response(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "neighbours_neighbours" => {
                            handle_neighbours_neighbours_request(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "neighbours_neighbours_reply" => {
                            handle_neighbours_neighbours_response(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "new_voronoi" => {
                            handle_new_voronoi_request(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "leave_voronoi" => {
                            handle_leave_voronoi_request(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        "leave_reply" => {
                            handle_leave_response(
                                payload,
                                &mut node_data,
                                session_clone,
                                zid,
                                cluster_name,
                            );
                        }
                        _ => println!("UNKNOWN NODE TOPIC"),
                    }
                    //ok ive updated node data send to API task
                    node_update_tx.send(node_data.clone()).unwrap();
                }
            }
        });

        let (api_requester_tx, api_requester_rx) = flume::bounded(32);
        let (api_responder_tx, api_responder_rx) = flume::bounded(32);

        async_std::task::spawn_blocking(move || {
            let mut node_data_copy = NodeData::new();
            loop {
                while let Ok(updated_node_data) = node_update_rx.try_recv() {
                    node_data_copy = updated_node_data.clone();
                }
                while let Ok(api_message) = api_requester_rx.try_recv() {
                    let api_response = match api_message {
                        ApiMessage::GetStatus => {
                            ApiResponse::GetStatusResponse(node_data_copy.status.clone())
                        }
                        ApiMessage::GetNeighbours => ApiResponse::GetNeighboursResponse(
                            node_data_copy.neighbours.clone().into_iter().collect(),
                        ),
                        ApiMessage::GetPolygon => {
                            ApiResponse::GetPolygonResponse(node_data_copy.polygon.clone())
                        }
                        ApiMessage::IsNeighbour(zid) => ApiResponse::IsNeighbourResponse(
                            node_data_copy.neighbours.contains_key(zid.as_str()),
                        ),
                    };
                    api_responder_tx.send(api_response).unwrap();
                }
            }
        });

        // async_std::task::spawn(async move {
        //     let mut node_data_copy = NodeData::new();
        //     loop {
        //         futures::select! {
        //         updated_node_data = node_update_rx.recv_async() => {
        //             if let Ok(updated_node_data) = updated_node_data {
        //                 node_data_copy = updated_node_data.clone();
        //             } else {
        //                 break; // Exit the loop if receiving from `node_update_rx` fails
        //             }
        //         }
        //         api_message = api_requester_rx.recv_async() => {
        //             if let Ok(api_message) = api_message {
        //                 let api_response = match api_message {
        //                     ApiMessage::GetStatus => {
        //                         ApiResponse::GetStatusResponse(node_data_copy.status.clone())
        //                     }
        //                 };
        //                 api_responder_tx.send(api_response).unwrap();
        //             } else {
        //                 break; // Exit the loop if receiving from `api_requester_rx` fails
        //             }
        //         }
        //     }
        //     }
        // });

        Self {
            session,
            cluster_name: cluster_name.to_string(),
            zid,
            api_requester_tx,
            api_responder_rx,
        }
    }

    // Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::node).
    pub fn join(&mut self) {
        let message = serialize(&DefaultMessage {
            sender_id: self.zid.clone(),
        })
        .unwrap();
        self.session
            .put(format!("{}/node/boot/new", self.cluster_name), message)
            .res_sync()
            .unwrap();
    }

    ///Node is dropped and leaves the cluster when not busy with task.
    pub fn leave(&mut self) {
        let status = self.get_status();
        if !matches!(status, NodeStatus::Leaving | NodeStatus::Offline) {
            let message = serialize(&DefaultMessage {
                sender_id: self.zid.clone(),
            })
            .unwrap();
            self.session
                .put(
                    format!("{}/node/boot/leave_request", self.cluster_name),
                    message,
                )
                .res_sync()
                .unwrap();
        }
    }

    ///End node when the user presses a key. Node is dropped and leaves the cluster.
    ///use to return Self but cant use builder pattern in C
    pub fn leave_on_pressed(&self, key: char) {
        let session = self.session.clone();
        let zid = self.zid.clone();
        let cluster = self.cluster_name.clone();
        task::spawn(async move {
            let mut buffer = [0; 1];
            loop {
                // Read a single byte from stdin
                if let Ok(()) = io::stdin().read_exact(&mut buffer).await {
                    if buffer[0] == key as u8 {
                        // Call the function when the user presses 'q'
                        let message = serialize(&DefaultMessage { sender_id: zid }).unwrap();
                        session
                            .put(format!("{}/node/boot/leave_request", cluster), message)
                            .res_sync()
                            .unwrap();
                        break;
                    }
                }
            }
        });
        //self
    }

    /// Get the zid of the node
    pub fn get_zid(&self) -> String {
        self.zid.clone()
    }
    ///Get node status
    pub fn get_status(&self) -> NodeStatus {
        self.api_requester_tx.send(ApiMessage::GetStatus).unwrap();
        if let ApiResponse::GetStatusResponse(status) = self.api_responder_rx.recv().unwrap() {
            status
        } else {
            panic!("Wrong response type");
        }
    }

    /// Get the neighbours of the node
    pub fn get_neighbours(&self) -> Vec<(String, (f64, f64))> {
        self.api_requester_tx
            .send(ApiMessage::GetNeighbours)
            .unwrap();
        if let ApiResponse::GetNeighboursResponse(neighbours) =
            self.api_responder_rx.recv().unwrap()
        {
            neighbours
        } else {
            panic!("Wrong response type");
        }
    }
    /// Check if the node is a neighbour
    pub fn is_neighbour(&self, zid: &str) -> bool {
        self.api_requester_tx
            .send(ApiMessage::IsNeighbour(zid.to_string()))
            .unwrap();
        if let ApiResponse::IsNeighbourResponse(bool) = self.api_responder_rx.recv().unwrap() {
            bool
        } else {
            panic!("Wrong response type");
        }
    }
    /// Get the polygon of the node
    pub fn get_polygon(&self) -> Vec<(f64, f64)> {
        self.api_requester_tx.send(ApiMessage::GetPolygon).unwrap();
        if let ApiResponse::GetPolygonResponse(polygon) = self.api_responder_rx.recv().unwrap() {
            polygon
        } else {
            panic!("Wrong response type");
        }
    }

    ///Check if the point site is in the polygon. Ray casting algorithm.
    pub fn is_in_polygon(&self, point: (f64, f64)) -> bool {
        let polygon;
        self.api_requester_tx.send(ApiMessage::GetPolygon).unwrap();
        if let ApiResponse::GetPolygonResponse(polygon_msg) = self.api_responder_rx.recv().unwrap()
        {
            polygon = polygon_msg;
        } else {
            panic!("Wrong response type");
        }
        if polygon.is_empty() {
            false
        } else {
            let mut i = 0;
            let mut j = polygon.len() - 1;
            let mut c = false;
            while i < polygon.len() {
                if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
                    && (point.0
                        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
                            / (polygon[j].1 - polygon[i].1)
                            + polygon[i].0)
                {
                    c = !c;
                }
                j = i;
                i += 1;
            }
            c
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
#[repr(C)]
pub enum NodeStatus {
    Joining,
    Online,
    Leaving,
    Offline,
}

impl NodeData {
    // Creates a new node instance with a [session](https://docs.rs/zenoh/0.7.0-rc/zenoh/struct.Session.html). Joins the cluster by messaging a boot node on that cluster.
    // Opens a subscription on topic `{cluster}/node/{zid}/*` to receive incoming messages.
    pub fn new() -> Self {
        Self {
            site: (-1., -1.),
            neighbours: OrderedMapPairs::new(),
            k_hop_neighbours: OrderedMapPairs::new(),
            polygon: vec![],
            received_counter: 0,
            expected_counter: -1,
            status: NodeStatus::Joining,
        }
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new()
    }
}

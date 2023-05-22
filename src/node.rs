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

use std::sync::{Arc, Mutex};
use std::thread;

pub use zenoh::prelude::sync::*;

/// A node in a network that has a point site which is used in the calculation of the voronoi diagram of a cluster. Computes its own voronoi polygon from its list of neighbours. Does not store information on entire cluster.
pub struct NodeData {
    pub(crate) site: (f64, f64),
    pub(crate) neighbours: OrderedMapPairs,
    pub(crate) k_hop_neighbours: OrderedMapPairs,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub(crate) polygon: Vec<(f64, f64)>,
    pub(crate) status: NodeStatus,
    pub(crate) cluster_name: String,
    pub(crate) zid: String,
}

pub struct Node {
    pub(crate) node_data: Arc<Mutex<NodeData>>,
    pub(crate) session: Arc<Session>,
    // subscription: Option<Subscriber<'a, ()>>,
}

impl Node {
    pub fn new(cluster: &str) -> Self {
        let session = zenoh::open(config::default())
            .res_sync()
            .unwrap()
            .into_arc();
        let zid = session.zid().to_string();
        let node_data = NodeData::new(cluster, zid);
        let node_data = Arc::new(Mutex::new(node_data));
        Self {
            node_data,
            session,
            //subscription: None,
        }
    }

    // Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::node).
    pub fn join(&mut self) {
        let node_data_clone = Arc::clone(&self.node_data);
        let session_clone = Arc::clone(&self.session);

        thread::spawn(move || {
            let guard = node_data_clone.lock().unwrap();
            let subscription = session_clone
                .declare_subscriber(format!("{}/node/{}/*", guard.cluster_name, guard.zid))
                .reliable()
                .res_sync()
                .unwrap();
            drop(guard);
            loop {
                //maybe dont break if offline just dont execute?
                if node_data_clone.lock().unwrap().status == NodeStatus::Offline {
                    break;
                }
                while let Ok(sample) = subscription.try_recv() {
                    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                    println!("Received message on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    let node_data_guard_clone = node_data_clone.lock().unwrap();

                    match topic {
                        "new_reply" => {
                            handle_owner_request(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "owner_request" => {
                            handle_owner_request(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "owner_response" => {
                            handle_owner_response(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "neighbours_neighbours" => {
                            handle_neighbours_neighbours_request(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "neighbours_neighbours_reply" => {
                            handle_neighbours_neighbours_response(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "new_voronoi" => {
                            handle_new_voronoi_request(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "leave_voronoi" => {
                            handle_leave_voronoi_request(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "leave_reply" => {
                            handle_leave_response(
                                payload,
                                node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        _ => println!("UNKNOWN NODE TOPIC"),
                    }
                    //thread::sleep(Duration::from_secs(2));
                }
            }
        });

        let node_data_guard = self.node_data.lock().unwrap();

        let message = serialize(&DefaultMessage {
            sender_id: node_data_guard.zid.clone(),
        })
        .unwrap();

        self.session
            .put(
                format!("{}/node/boot/new", node_data_guard.cluster_name),
                message,
            )
            .res_sync()
            .unwrap();

        //self.subscription = Some(subscription);
    }

    ///Node is dropped and leaves the cluster when not busy with task.
    pub fn leave(&mut self) {
        let mut node_data_guard = self.node_data.lock().unwrap();
        if !matches!(
            node_data_guard.status,
            NodeStatus::Leaving | NodeStatus::Offline
        ) {
            node_data_guard.status = NodeStatus::Leaving;
            let message = serialize(&DefaultMessage {
                sender_id: node_data_guard.zid.clone(),
            })
            .unwrap();
            self.session
                .put(
                    format!("{}/node/boot/leave_request", node_data_guard.cluster_name),
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
        let zid = self.node_data.lock().unwrap().zid.clone();
        let cluster = self.node_data.lock().unwrap().cluster_name.clone();
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
        self.node_data.lock().unwrap().zid.clone()
    }
    ///Get node status
    pub fn get_status(&self) -> NodeStatus {
        self.node_data.lock().unwrap().status.clone()
    }
    /// Get the neighbours of the node
    pub fn get_neighbours(&self) -> Vec<(String, (f64, f64))> {
        self.node_data
            .lock()
            .unwrap()
            .neighbours
            .clone()
            .into_iter()
            .collect()
    }
    /// Check if the node is a neighbour
    pub fn is_neighbour(&self, zid: &str) -> bool {
        self.node_data.lock().unwrap().neighbours.contains_key(zid)
    }
    /// Get the polygon of the node
    pub fn get_polygon(&self) -> Vec<(f64, f64)> {
        self.node_data.lock().unwrap().polygon.clone()
    }

    ///Check if the point site is in the polygon. Ray casting algorithm.
    pub fn is_in_polygon(&self, point: (f64, f64)) -> bool {
        let guard = self.node_data.lock().unwrap();
        if guard.polygon.is_empty() {
            false
        } else {
            let mut i = 0;
            let mut j = guard.polygon.len() - 1;
            let mut c = false;
            while i < guard.polygon.len() {
                if ((guard.polygon[i].1 > point.1) != (guard.polygon[j].1 > point.1))
                    && (point.0
                        < (guard.polygon[j].0 - guard.polygon[i].0)
                            * (point.1 - guard.polygon[i].1)
                            / (guard.polygon[j].1 - guard.polygon[i].1)
                            + guard.polygon[i].0)
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
    pub fn new(cluster: &str, zid: String) -> Self {
        Self {
            site: (-1., -1.),
            neighbours: OrderedMapPairs::new(),
            k_hop_neighbours: OrderedMapPairs::new(),
            polygon: vec![],
            received_counter: 0,
            expected_counter: -1,
            cluster_name: cluster.to_string(),
            status: NodeStatus::Joining,
            zid,
        }
    }
}

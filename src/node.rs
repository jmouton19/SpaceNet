use crate::handlers::node::handle_join_response::handle_join_response;
use crate::handlers::node::handle_leave_neighbours_neighbours_request::handle_leave_neighbours_neighbours_request;
use crate::handlers::node::handle_leave_neighbours_neighbours_response::handle_leave_neighbours_neighbours_response;
use crate::handlers::node::handle_leave_response::handle_leave_response;
use crate::handlers::node::handle_leave_voronoi_request::handle_leave_voronoi_request;
use crate::handlers::node::handle_neighbours_neighbours_request::handle_neighbours_neighbours_request;
use crate::handlers::node::handle_neighbours_neighbours_response::handle_neighbours_neighbours_response;
use crate::handlers::node::handle_neighbours_request::handle_neighbours_request;
use crate::handlers::node::handle_new_voronoi_request::handle_new_voronoi_request;
use crate::handlers::node::set_expected_neighbours::set_expected_neighbours;
use crate::message::DefaultMessage;
use crate::types::OrderedMapPairs;
use async_std::io::ReadExt;
use async_std::sync::Arc;
use async_std::{io, task};
use bincode::serialize;
use std::thread;
use std::time::Duration;
pub use zenoh::prelude::sync::*;
use zenoh::subscriber::Subscriber;

/// A node in a network that has a point site which is used in the calculation of the voronoi diagram of a cluster. Computes its own voronoi polygon from its list of neighbours. Does not store information on entire cluster.
pub struct Node<'a> {
    pub(crate) cluster_name: String,
    pub(crate) session: Arc<Session>,
    pub(crate) site: (f64, f64),
    pub(crate) neighbours: OrderedMapPairs,
    pub(crate) zid: String,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub(crate) status: NodeStatus,
    pub(crate) polygon: Vec<(f64, f64)>,
    subscription: Subscriber<'a, flume::Receiver<Sample>>,
}

#[derive(PartialEq, Clone, Debug)]
#[repr(C)]
pub enum NodeStatus {
    Joining,
    Online,
    Leaving,
    Offline,
}

impl Node<'_> {
    /// Creates a new node instance with a [session](https://docs.rs/zenoh/0.7.0-rc/zenoh/struct.Session.html). Joins the cluster by messaging a boot node on that cluster.
    /// Opens a subscription on topic `{cluster}/node/{zid}/*` to receive incoming messages.
    pub fn new(cluster: &str) -> Self {
        let session = zenoh::open(Config::default()).res().unwrap().into_arc();
        let zid = session.zid().to_string();
        let node_subscription = session
            .declare_subscriber(format!("{}/node/{}/*", cluster, zid))
            .reliable()
            .res()
            .unwrap();

        let message = serialize(&DefaultMessage {
            sender_id: zid.clone(),
        })
        .unwrap();
        session
            .put(format!("{}/node/boot/new", cluster), message)
            .res()
            .unwrap();
        Self {
            cluster_name: cluster.to_string(),
            zid,
            session,
            site: (-1., -1.),
            neighbours: OrderedMapPairs::new(),
            polygon: vec![],
            received_counter: 0,
            expected_counter: -1,
            status: NodeStatus::Joining,
            subscription: node_subscription,
        }
    }

    // pub fn join(& self){
    //     let message = json!(DefaultMessage {
    //         sender_id: self.zid.clone(),
    //     });
    //     self.session.put(format!("{}/node/boot/new",self.cluster), message).res().unwrap();
    // }

    /// Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::node).
    pub fn run(&mut self) {
        while let Ok(sample) = self.subscription.try_recv() {
            if self.status == NodeStatus::Offline {
                break;
            }
            let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
            println!("Received message on topic... {:?}", topic);
            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
            //sleep 1 seconds to delay message
            //thread::sleep(Duration::from_secs(1));

            match topic {
                "new_reply" => {
                    handle_join_response(payload, self);
                }
                "new_neighbours" => {
                    handle_neighbours_request(payload, self);
                }
                "neighbours_expected" => {
                    set_expected_neighbours(payload, self);
                }
                "neighbours_neighbours" => {
                    handle_neighbours_neighbours_request(payload, self);
                }
                "leave_neighbours_neighbours" => {
                    handle_leave_neighbours_neighbours_request(payload, self);
                }
                "neighbours_neighbours_reply" => {
                    handle_neighbours_neighbours_response(payload, self);
                }
                "Leave_neighbours_neighbours_reply" => {
                    handle_leave_neighbours_neighbours_response(payload, self);
                }
                "new_voronoi" => {
                    handle_new_voronoi_request(payload, self);
                }
                "leave_voronoi" => {
                    handle_leave_voronoi_request(payload, self);
                }
                "leave_reply" => {
                    handle_leave_response(payload, self);
                }
                _ => println!("UNKNOWN NODE TOPIC"),
            }
        }
    }

    // pub async fn run_async(&mut self) {
    //         while self.status != NodeStatus::Offline {
    //             self.run();
    //         }
    // }

    /// End node when the user presses a key. Node is dropped and leaves the cluster.
    // use to return Self but cant use builder pattern in C
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
                            .res()
                            .unwrap();
                        break;
                    }
                }
            }
        });
        //self
    }

    ///  Node is dropped and leaves the cluster when not busy with task.
    pub fn leave(&mut self) {
        if !matches!(self.status, NodeStatus::Leaving | NodeStatus::Offline) {
            self.status = NodeStatus::Leaving;
            let message = serialize(&DefaultMessage {
                sender_id: self.zid.clone(),
            })
            .unwrap();
            self.session
                .put(
                    format!("{}/node/boot/leave_request", self.cluster_name),
                    message,
                )
                .res()
                .unwrap();
        }
    }

    /// Get the zid of the node
    pub fn get_zid(&self) -> &str {
        self.zid.as_str()
    }
    ///Get node status
    pub fn get_status(&self) -> NodeStatus {
        self.status.clone()
    }

    /// Get the neighbours of the node
    pub fn get_neighbours(&self) -> Vec<(String, (f64, f64))> {
        self.neighbours.clone().into_iter().collect()
    }
    /// Check if the node is a neighbour
    pub fn is_neighbour(&self, zid: &str) -> bool {
        self.neighbours.contains_key(zid)
    }
    /// Get the polygon of the node
    pub fn get_polygon(&self) -> Vec<(f64, f64)> {
        self.polygon.clone()
    }

    /// Check if the point site is in the polygon. Ray casting algorithm.
    pub fn is_in_polygon(&self, point: (f64, f64)) -> bool {
        if self.polygon.is_empty() {
            false
        } else {
            let mut i = 0;
            let mut j = self.polygon.len() - 1;
            let mut c = false;
            while i < self.polygon.len() {
                if ((self.polygon[i].1 > point.1) != (self.polygon[j].1 > point.1))
                    && (point.0
                        < (self.polygon[j].0 - self.polygon[i].0) * (point.1 - self.polygon[i].1)
                            / (self.polygon[j].1 - self.polygon[i].1)
                            + self.polygon[i].0)
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

// pub async fn runner_async(node: Arc<Mutex<Node<'_>>>) {
//     loop {
//         let mut node_guard = node.lock().await;
//         if node_guard.status == NodeStatus::Offline {
//             break;
//         }
//         node_guard.run();
//         drop(node_guard);
//     }
// }

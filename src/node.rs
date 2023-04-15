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
    pub(crate) running: bool,
    pub(crate) polygon: Vec<(f64, f64)>,
    subscription: Subscriber<'a, flume::Receiver<Sample>>,
}

impl Node<'_> {
    /// Creates a new node instance with a [session](https://docs.rs/zenoh/0.7.0-rc/zenoh/struct.Session.html). Joins the cluster by messaging a boot node on that cluster.
    /// Opens a subscription on topic `{cluster}/node/{zid}/*` to receive incoming messages.
    pub fn new(config: Config, cluster: &str) -> Self {
        let session = zenoh::open(config).res().unwrap().into_arc();
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
            running: true,
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
            if !self.running {
                break;
            }
            let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
            println!("Received message on topic... {:?}", topic);
            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();

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

    /// End node when the user presses a key. Node is dropped and leaves the cluster.
    pub fn leave_on_pressed(self, key: char) -> Self {
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
        self
    }
    ///  Node is dropped and leaves the cluster.
    pub fn leave(self) {
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
    /// Get the zid of the node
    pub fn get_zid(&self) -> &str {
        self.zid.as_str()
    }
    /// Get the neighbours of the node
    pub fn get_neighbours(&self) -> OrderedMapPairs {
        self.neighbours.clone()
    }
    /// Get the polygon of the node
    pub fn get_polygon(&self) -> Vec<(f64, f64)> {
        self.polygon.clone()
    }
    /// Check if the node is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}

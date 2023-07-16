use crate::handlers::node::node_api_matcher::{node_api_matcher, ApiMessage, ApiResponse};
use crate::handlers::node::node_topic_matcher::node_topic_matcher;
use crate::message::{DefaultMessage, NewVoronoiRequest};
use crate::types::OrderedMapPairs;
use async_std::io::ReadExt;
use async_std::{io, task};
use bincode::serialize;
use std::collections::HashMap;

use std::sync::Arc;

use crate::payload_message::PayloadMessage;
use crate::sse::{Initialize, Player};
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
    pub(crate) players: HashMap<String, (f64, f64)>,
    pub(crate) status: NodeStatus,
}

pub struct Node {
    pub(crate) session: Arc<Session>,
    pub(crate) cluster_name: String,
    pub(crate) zid: String,
    pub(crate) api_requester_tx: flume::Sender<ApiMessage>,
    pub(crate) api_responder_rx: flume::Receiver<ApiResponse>,
}

impl Node {
    pub fn new(cluster_name: &str) -> Self {
        let session = zenoh::open(config::default())
            .res_sync()
            .unwrap()
            .into_arc();
        let zid = session.zid().to_string();

        let session_clone = Arc::clone(&session);
        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = zid.to_string();
        let (zenoh_tx, zenoh_rx) = flume::unbounded();
        let (sse_tx, sse_rx) = flume::unbounded();
        //task to listen for zenoh messages, sends message to processor task.
        async_std::task::spawn(async move {
            let subscriber = session_clone
                .declare_subscriber(format!("{}/node/{}/*", cluster_name_clone, zid_clone))
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            let sse_subscriber = session_clone
                .declare_subscriber(format!("{}/sse/get/*", cluster_name_clone))
                .with(flume::unbounded())
                .res_async()
                .await
                .unwrap();
            loop {
                futures::select! {
                       sample = subscriber.recv_async() => {
                        if let Ok(sample) = sample {
                            zenoh_tx.send(sample).unwrap();
                        } else {
                            break;
                        }
                    }
                    sample = sse_subscriber.recv_async() => {
                        if let Ok(sample) = sample {
                              sse_tx.send(sample).unwrap();
                        } else {
                            break;
                        }
                    }
                }
            }
        });
        // async_std::task::spawn(async move {
        //     let subscriber = session_clone
        //         .declare_subscriber(format!("{}/node/{}/*", cluster_name_clone, zid_clone))
        //         .with(flume::unbounded())
        //         .reliable()
        //         .res_async()
        //         .await
        //         .unwrap();
        //     while let Ok(sample) = subscriber.recv_async().await {
        //         zenoh_tx.send(sample).unwrap();
        //     }
        // });

        //Processor task, handles messages from zenoh. Update node_data copy in API task
        let session_clone = Arc::clone(&session);
        let cluster_name_clone = cluster_name.to_string();
        let zid_clone = zid.to_string();
        let (api_requester_tx, api_requester_rx) = flume::unbounded();
        let (api_responder_tx, api_responder_rx) = flume::unbounded();
        async_std::task::spawn_blocking(move || {
            let mut node_data = NodeData::new();
            loop {
                while let Ok(sample) = zenoh_rx.try_recv() {
                    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                    println!("Received message on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    node_topic_matcher(
                        topic,
                        payload,
                        &mut node_data,
                        &session_clone,
                        zid_clone.as_str(),
                        cluster_name_clone.as_str(),
                    );
                }
                if let Ok(api_message) = api_requester_rx.try_recv() {
                    node_api_matcher(
                        api_message,
                        &mut node_data,
                        &api_responder_tx,
                        &session_clone,
                        cluster_name_clone.as_str(),
                        zid_clone.as_str(),
                    );
                }
                if let Ok(sample) = sse_rx.try_recv() {
                    let sse_id = sample.key_expr.split('/').nth(3).unwrap_or("");
                    let players: Vec<Player> = node_data
                        .players
                        .clone()
                        .into_iter()
                        .map(|(player_id, (x, y))| Player { player_id, x, y })
                        .collect();
                    let initial_message = Initialize {
                        players,
                        polygon: node_data.polygon.clone(),
                        site: node_data.site,
                        sender_id: zid_clone.clone(),
                    };

                    let message = serialize(&initial_message).unwrap();
                    session_clone
                        .put(
                            format!("{}/sse/event/initialize/{}", cluster_name_clone, sse_id),
                            message,
                        )
                        .res_sync()
                        .unwrap();
                }
            }
        });
        Self {
            session,
            cluster_name: cluster_name.to_string(),
            zid,
            api_requester_tx,
            api_responder_rx,
        }
    }

    // Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::node).
    pub fn join(&mut self, site: (f64, f64)) {
        self.set_site(site);
        self.set_status(NodeStatus::Joining);
        let message = serialize(&NewVoronoiRequest {
            sender_id: self.zid.clone(),
            site: self.get_site(),
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

    pub fn get_site(&self) -> (f64, f64) {
        self.api_requester_tx.send(ApiMessage::GetSite).unwrap();
        if let ApiResponse::GetSiteResponse(site) = self.api_responder_rx.recv().unwrap() {
            site
        } else {
            panic!("Wrong response type");
        }
    }

    fn set_status(&self, status: NodeStatus) {
        self.api_requester_tx
            .send(ApiMessage::SetStatus(status))
            .unwrap();
    }

    fn set_site(&self, site: (f64, f64)) {
        self.api_requester_tx
            .send(ApiMessage::SetSite(site))
            .unwrap();
    }

    pub fn add_player(&self, player_id: &str) {
        let (x, y) = self.get_site();
        let player = Player {
            player_id: player_id.to_string(),
            x,
            y,
        };
        self.api_requester_tx
            .send(ApiMessage::AddPlayer(player))
            .unwrap();
    }
    pub fn update_player(&self, player_id: &str, x: f64, y: f64) {
        let player = Player {
            player_id: player_id.to_string(),
            x,
            y,
        };
        self.api_requester_tx
            .send(ApiMessage::UpdatePlayer(player))
            .unwrap();
    }
    pub fn remove_player(&self, player_id: &str) {
        self.api_requester_tx
            .send(ApiMessage::RemovePlayer(player_id.to_string()))
            .unwrap();
    }

    pub fn closest_neighbour(&self, point: (f64, f64)) -> String {
        let neighbours = self.get_neighbours_sites();
        let mut min_distance = f64::MAX;
        let mut min_neighbour = "";
        for (neighbour, site) in neighbours.iter() {
            let dx = site.0 - point.0;
            let dy = site.1 - point.1;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < min_distance {
                min_distance = distance;
                min_neighbour = neighbour;
            }
        }
        min_neighbour.to_string()
    }

    pub fn send_message(&self, payload: Vec<u8>, topic: &str) {
        let message = serialize(&PayloadMessage {
            payload,
            sender_id: self.zid.clone(),
            topic: topic.to_string(),
        })
        .unwrap();
        self.session.put(topic, message).res_sync().unwrap();
    }

    /// Get the neighbours of the node
    pub fn get_neighbours_sites(&self) -> Vec<(String, (f64, f64))> {
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

    pub fn get_neighbours(&self) -> Vec<String> {
        self.api_requester_tx
            .send(ApiMessage::GetNeighbours)
            .unwrap();
        if let ApiResponse::GetNeighboursResponse(neighbours) =
            self.api_responder_rx.recv().unwrap()
        {
            neighbours.iter().map(|(s, _)| s.clone()).collect()
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
            players: HashMap::new(),
            received_counter: 0,
            expected_counter: -1,
            status: NodeStatus::Offline,
        }
    }
}

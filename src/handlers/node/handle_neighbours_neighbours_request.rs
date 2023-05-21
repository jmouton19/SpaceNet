use crate::message::{DefaultMessage, NeighboursResponse};
use crate::node::{Node, NodeData, SyncResolve};
use bincode::{deserialize, serialize};
use std::sync::{Arc, MutexGuard};
use zenoh::Session;

/// Sends list of neighbours to new node
pub fn handle_neighbours_neighbours_request(
    payload: &[u8],
    node_data: MutexGuard<NodeData>,
    session: Arc<Session>,
) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //send list of neighbours back to new node
    let message = serialize(&NeighboursResponse {
        sender_id: node_data.zid.clone(),
        neighbours: node_data.neighbours.clone(),
    })
    .unwrap();
    session
        .put(
            format!(
                "{}/node/{}/neighbours_neighbours_reply",
                node_data.cluster_name, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();
}

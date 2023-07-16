use crate::message::{DefaultMessage, NeighboursResponse};
use crate::node::{NodeData, SyncResolve};
use bincode::{deserialize, serialize};

use std::sync::Arc;
use zenoh::Session;

/// Sends list of neighbours to new node
pub fn handle_neighbours_neighbours_request(
    payload: &[u8],
    node_data: &mut NodeData,
    session: &Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //send list of neighbours back to new node
    let message = serialize(&NeighboursResponse {
        sender_id: zid.to_string(),
        neighbours: node_data.neighbours.clone(),
    })
    .unwrap();
    session
        .put(
            format!(
                "{}/node/{}/neighbours_neighbours_reply",
                cluster_name, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();
}

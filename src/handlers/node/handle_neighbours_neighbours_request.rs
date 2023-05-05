use crate::message::{DefaultMessage, NeighboursResponse};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

/// Sends list of neighbours to new node
pub fn handle_neighbours_neighbours_request(payload: &[u8], node: &mut Node) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //send list of neighbours back to new node
    let message = serialize(&NeighboursResponse {
        sender_id: node.zid.clone(),
        neighbours: node.neighbours.clone(),
    })
    .unwrap();
    node.session
        .put(
            format!(
                "{}/node/{}/neighbours_neighbours_reply",
                node.cluster_name, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();
}

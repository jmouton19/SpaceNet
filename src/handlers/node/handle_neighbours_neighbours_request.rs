use crate::message::{NeighboursNeighboursRequest, NeighboursResponse};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

pub fn handle_neighbours_neighbours_request(payload: &[u8], node: &mut Node) {
    let data: NeighboursNeighboursRequest = deserialize(payload).unwrap();
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
                node.cluster_name, data.new_zid
            ),
            message,
        )
        .res()
        .unwrap();
}

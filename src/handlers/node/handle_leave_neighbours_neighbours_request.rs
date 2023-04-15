use crate::message::{DefaultMessage, NeighboursResponse};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

pub fn handle_leave_neighbours_neighbours_request(payload: &[u8], node: &mut Node) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //send list of neighbours back to leaver
    let message = serialize(&NeighboursResponse {
        sender_id: node.zid.clone(),
        neighbours: node.neighbours.clone(),
    })
    .unwrap();
    node.session
        .put(
            format!(
                "{}/node/{}/Leave_neighbours_neighbours_reply",
                node.cluster_name, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();
}

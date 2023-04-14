use crate::message::DefaultMessage;
use crate::node::{Node, SyncResolve};
use bincode::serialize;

pub fn handle_leave_response(payload: &[u8], node: &mut Node) {
    //tell me how many to wait for
    node.expected_counter = node.neighbours.len() as i32;
    println!(
        "Expecting {} replies... before i leave",
        node.expected_counter
    );

    //get FULL neighbour list
    //request neighbours from neighbours and send it back to me
    let message = serialize(&DefaultMessage {
        sender_id: node.zid.clone(),
    })
    .unwrap();
    for neighbour_id in node.neighbours.keys() {
        node.session
            .put(
                format!(
                    "{}/node/{}/leave_neighbours_neighbours",
                    node.cluster, neighbour_id
                ),
                message.clone(),
            )
            .res()
            .unwrap();
    }
}

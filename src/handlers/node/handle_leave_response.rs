use crate::message::{DefaultMessage, ExpectedNodes};
use crate::node::{Node, SyncResolve};
use bincode::serialize;

pub fn handle_leave_response(_payload: &[u8], node: &mut Node) {
    //tell me how many to wait for
    node.expected_counter = node.neighbours.len() as i32;
    println!(
        "Expecting {} replies... before i leave",
        node.expected_counter
    );

    if node.neighbours.is_empty() {
        let message = serialize(&ExpectedNodes {
            number: 0,
            sender_id: node.zid.clone(),
        })
        .unwrap();
        node.session
            .put(
                format!("{}/counter/expected_wait", node.cluster_name),
                message,
            )
            .res()
            .unwrap();
        println!("IM SHUTTING DOWN BOOT! - i have no friends ;/");
        node.running = false;
        let _ = node;
    } else {
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
                        node.cluster_name, neighbour_id
                    ),
                    message.clone(),
                )
                .res()
                .unwrap();
        }
    }
}

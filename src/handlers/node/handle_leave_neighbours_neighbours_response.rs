use crate::message::{ExpectedNodes, NeighboursResponse};
use crate::node::{Node, NodeStatus, SyncResolve};
use bincode::{deserialize, serialize};

/// Combine all neighbour lists received. If number of messages received is equal to the expected number tell all neighbours to calculate new voronoi without leavers site.
/// Drop leaver node instance.
pub fn handle_leave_neighbours_neighbours_response(payload: &[u8], node: &mut Node) {
    let data: NeighboursResponse = deserialize(payload).unwrap();
    node.neighbours.extend(data.neighbours);
    node.received_counter += 1;
    println!(
        "Message received from {}....  expecting {} more.",
        data.sender_id,
        node.expected_counter - node.received_counter
    );
    if node.expected_counter == node.received_counter {
        node.neighbours.remove(node.zid.as_str());
        node.received_counter = 0;
        node.expected_counter = -1;

        //tell boot how many to wait for
        //+1 if wait for node to say its left
        let message = serialize(&ExpectedNodes {
            number: node.neighbours.len() as i32,
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

        //tell all neighbours to calc new voronoi without my site.
        let message = serialize(&NeighboursResponse {
            neighbours: node.neighbours.clone(),
            sender_id: node.zid.clone(),
        })
        .unwrap();
        for neighbour_id in node.neighbours.keys() {
            node.session
                .put(
                    format!("{}/node/{}/leave_voronoi", node.cluster_name, neighbour_id),
                    message.clone(),
                )
                .res()
                .unwrap();
        }

        //drop node instance
        println!("IM SHUTTING DOWN BOOT!");
        // let message = serialize(&DefaultMessage{
        // sender_id:node.zid.clone()});
        // node.session.put("counter/leaving", message.clone()).res().unwrap();
        node.status = NodeStatus::Offline;
        //let _ = node;
    } //else do nothing
}

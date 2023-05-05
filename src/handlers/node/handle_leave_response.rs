use crate::message::{ExpectedNodes, NeighboursResponse};
use crate::node::{Node, NodeStatus, SyncResolve};
use bincode::serialize;

/// Handles leave response, if no neighbours, shut down, else request neighbour list from neighbours
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

        node.status = NodeStatus::Offline;
        //let _ = node;
    } else {
        //send me neighbours my neighbors and without me
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
    }
}

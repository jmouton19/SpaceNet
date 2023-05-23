use crate::message::{ExpectedNodes, NeighboursResponse};
use crate::node::{NodeData, NodeStatus, SyncResolve};
use bincode::serialize;

use std::sync::Arc;
use zenoh::Session;

/// Handles leave response, if no neighbours, shut down, else tell neighbours to recalculate voronoi without me but with my neighbours.
pub fn handle_leave_response(
    _payload: &[u8],
    node_data: &mut NodeData,
    session: Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    node_data.status = NodeStatus::Leaving;
    //tell me how many to wait for
    node_data.expected_counter = node_data.neighbours.len() as i32;
    println!(
        "Expecting {} replies... before i leave",
        node_data.expected_counter
    );

    if node_data.neighbours.is_empty() {
        let message = serialize(&ExpectedNodes {
            number: 0,
            sender_id: zid.to_string(),
        })
        .unwrap();
        session
            .put(format!("{}/counter/expected_wait", cluster_name), message)
            .res()
            .unwrap();
        println!("IM SHUTTING DOWN BOOT! - i have no friends ;/");

        node_data.status = NodeStatus::Offline;
        //let _ = node;
    } else {
        //send me neighbours my neighbors and without me
        let message = serialize(&ExpectedNodes {
            number: node_data.neighbours.len() as i32,
            sender_id: zid.to_string(),
        })
        .unwrap();
        session
            .put(format!("{}/counter/expected_wait", cluster_name), message)
            .res()
            .unwrap();

        let message = serialize(&NeighboursResponse {
            neighbours: node_data.neighbours.clone(),
            sender_id: zid.to_string(),
        })
        .unwrap();
        for neighbour_id in node_data.neighbours.keys() {
            session
                .put(
                    format!("{}/node/{}/leave_voronoi", cluster_name, neighbour_id),
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
        node_data.status = NodeStatus::Offline;
        //let _ = node;
    }
}

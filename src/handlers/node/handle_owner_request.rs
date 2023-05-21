use crate::message::{NewNodeResponse, OwnerResponse};
use crate::node::{Node, NodeData, SyncResolve};
use bincode::{deserialize, serialize};
use std::sync::{Arc, MutexGuard};
use zenoh::Session;

/// Passes on itself, its neighbours and new site point to new node.
pub fn handle_owner_request(
    payload: &[u8],
    mut node_data: MutexGuard<NodeData>,
    session: Arc<Session>,
) {
    let data: NewNodeResponse = deserialize(payload).unwrap();
    println!(
        "New node at site... {:?} with id... {:?}",
        data.new_site, data.new_id
    );

    if node_data.zid == data.new_id {
        node_data.site = data.new_site;
    }

    // send my point & my neighbors to new node with new site
    let message = serialize(&OwnerResponse {
        sender_id: node_data.zid.clone(),
        sender_site: node_data.site,
        new_site: data.new_site,
        neighbours: node_data.neighbours.clone(),
    })
    .unwrap();

    session
        .put(
            format!(
                "{}/node/{}/owner_response",
                node_data.cluster_name, data.new_id
            ),
            message,
        )
        .res()
        .unwrap();
}

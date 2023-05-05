use crate::message::{NewNodeResponse, OwnerResponse};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

/// Sets site given from boot node and messages land owner to request neighbour list
pub fn handle_owner_request(payload: &[u8], node: &mut Node) {
    let data: NewNodeResponse = deserialize(payload).unwrap();
    println!(
        "New node at site... {:?} with id... {:?}",
        data.new_site, data.new_id
    );

    if node.zid == data.new_id {
        node.site = data.new_site;
    }

    // send my point & my neighbors to new node with new site
    let message = serialize(&OwnerResponse {
        sender_id: node.zid.clone(),
        sender_site: node.site,
        new_site: data.new_site,
        neighbours: node.neighbours.clone(),
    })
    .unwrap();

    node.session
        .put(
            format!("{}/node/{}/owner_response", node.cluster_name, data.new_id),
            message,
        )
        .res()
        .unwrap();
}

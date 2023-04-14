use crate::message::{NewNodeResponse, NewVoronoiRequest};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

pub fn handle_join_response(payload: &[u8], node: &mut Node) {
    let data: NewNodeResponse = deserialize(payload.as_ref()).unwrap();
    println!(
        "New point.... {:?} owner... {:?}",
        data.new_site, data.land_owner
    );
    //set site to given site
    node.site = data.new_site;

    node.neighbours
        .insert(data.land_owner.clone(), data.land_owner_site);
    //request neighbour list from land owner
    let message = serialize(&NewVoronoiRequest {
        sender_id: node.zid.clone(),
        site: node.site,
    })
    .unwrap();
    node.session
        .put(
            format!("{}/node/{}/new_neighbours", node.cluster, data.land_owner),
            message,
        )
        .res()
        .unwrap();
}

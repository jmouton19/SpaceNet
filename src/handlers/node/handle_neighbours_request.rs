use crate::message::{
    ExpectedNodes, NeighboursNeighboursRequest, NeighboursResponse, NewVoronoiRequest,
};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

pub fn handle_neighbours_request(payload: &[u8], node: &mut Node) {
    let data: NewVoronoiRequest = deserialize(payload.as_ref()).unwrap();
    println!(
        "New point at site... {:?} from... {:?}",
        data.site, data.sender_id
    );

    let neigh_len = node.neighbours.len() as i32;

    //tell new node how many to wait for
    let message = serialize(&ExpectedNodes {
        number: neigh_len + 1,
        sender_id: node.zid.clone(),
    })
    .unwrap();
    node.session
        .put(
            format!(
                "{}/node/{}/neighbours_expected",
                node.cluster, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();

    //send my neighbors
    let message = serialize(&NeighboursResponse {
        sender_id: node.zid.clone(),
        neighbours: node.neighbours.clone(),
    })
    .unwrap();
    node.session
        .put(
            format!(
                "{}/node/{}/neighbours_neighbours_reply",
                node.cluster, data.sender_id
            ),
            message,
        )
        .res()
        .unwrap();

    //request neighbours from neighbours and send it to new node
    let message = serialize(&NeighboursNeighboursRequest {
        new_zid: data.sender_id,
        sender_id: node.zid.clone(),
    })
    .unwrap();
    for neighbour_id in node.neighbours.keys() {
        node.session
            .put(
                format!(
                    "{}/node/{}/neighbours_neighbours",
                    node.cluster, neighbour_id
                ),
                message.clone(),
            )
            .res()
            .unwrap();
    }
}

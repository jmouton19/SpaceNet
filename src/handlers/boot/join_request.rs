use crate::message::{DefaultMessage, NewNodeResponse};
use crate::node::{Node, SyncResolve};
use crate::types::{closest_point, OrderedMapPairs, OrderedMapPolygon};
use bincode::{deserialize, serialize};
use rand::Rng;

pub fn join_request(
    payload: &[u8],
    node: &mut Node,
    polygon_list: &mut OrderedMapPolygon,
    cluster: &mut OrderedMapPairs,
) {
    //let data: DefaultMessage = deserialize(payload.as_ref()).unwrap();
    let data: DefaultMessage = deserialize(payload).unwrap();

    //get random point to give to new node
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple

    println!("------------------------------------");
    println!("Giving point {:?}.... to {:?}", point, data.sender_id);
    println!("------------------------------------");

    //find closest node to new point
    let (land_owner_site, land_owner) = closest_point(cluster, point);
    println!("{}", land_owner);

    //add node to cluster
    cluster.insert(data.sender_id.to_string(), point);
    polygon_list.insert(data.sender_id.to_string(), vec![]);

    let json_message = serialize(&NewNodeResponse {
        new_site: point,
        land_owner,
        land_owner_site,
        sender_id: node.zid.clone(),
    })
    .unwrap();

    let _ = node
        .session
        .put(
            format!("{}/node/{}/new_reply", node.cluster, data.sender_id),
            json_message,
        )
        .res();
}

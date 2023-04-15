use crate::boot_node::BootNode;
use crate::message::{DefaultMessage, NewNodeResponse};
use crate::node::SyncResolve;
use crate::types::closest_point;
use bincode::{deserialize, serialize};
use rand::Rng;

pub fn handle_join_request(payload: &[u8], boot_node: &mut BootNode) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //get random point to give to new node
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple

    println!("------------------------------------");
    println!("Giving point {:?}.... to {:?}", point, data.sender_id);
    println!("------------------------------------");

    //find closest node to new point
    if boot_node.cluster.is_empty() {
        boot_node.cluster.insert(data.sender_id.to_string(), point);
        boot_node
            .polygon_list
            .insert(data.sender_id.to_string(), vec![]);
    }
    let (land_owner_site, land_owner) = closest_point(&boot_node.cluster, point);
    println!("{}", land_owner);

    //add node to cluster
    boot_node.cluster.insert(data.sender_id.to_string(), point);
    boot_node
        .polygon_list
        .insert(data.sender_id.to_string(), vec![]);

    let json_message = serialize(&NewNodeResponse {
        new_site: point,
        land_owner,
        land_owner_site,
        sender_id: boot_node.zid.clone(),
    })
    .unwrap();

    let _ = boot_node
        .session
        .put(
            format!(
                "{}/node/{}/new_reply",
                boot_node.cluster_name, data.sender_id
            ),
            json_message,
        )
        .res();
}

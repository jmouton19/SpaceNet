use crate::boot_node::BootNode;
use crate::message::{DefaultMessage, NewNodeResponse};
use crate::node::SyncResolve;
use crate::types::{closest_point, point_within_distance};
use bincode::{deserialize, serialize};
use rand::Rng;

/// Handles a join request from a new node. Boot node assigns a point to the new node and states the closest node (`land_owner`) to the new point. Sends the new node the zid of the 'land_owner' node.
/// Adds new node to cluster and polygon list of boot node.
pub fn handle_join_request(payload: &[u8], boot_node: &mut BootNode) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //get random point to give to new node
    let mut rng = rand::thread_rng();
    let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple

    println!("------------------------------------");
    println!("Giving point {:?}.... to {:?}", point, data.sender_id);
    println!("------------------------------------");

    //find closest node to new point
    if boot_node.cluster.is_empty() {
        point = (50.0, 50.0);
        boot_node.cluster.insert(data.sender_id.to_string(), point);
        boot_node
            .polygon_list
            .insert(data.sender_id.to_string(), vec![]);
    } else {
        //check if a point exist in boot_node.cluster.values that is within X distance of the new point if so precalculate the new point
        let mut point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
        let tolerance = 0.01;
        while point_within_distance(&boot_node.cluster, point, tolerance) {
            point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
        }
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

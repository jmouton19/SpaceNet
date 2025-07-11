use crate::boot_node::BootNodeData;
use crate::message::{NewNodeResponse, NewVoronoiRequest};
use crate::node::SyncResolve;
use crate::types::{closest_point, point_within_distance};
use bincode::{deserialize, serialize};
use rand::Rng;
use std::sync::Arc;
use zenoh::Session;

/// Handles a join request from a new node. Boot node assigns a point to the new node and states the closest node (`land_owner`) to the new point. Sends the new node to the 'land_owner' node.
/// Adds new node to cluster and polygon list of boot node.
pub fn handle_join_request(
    payload: &[u8],
    boot_node_data: &mut BootNodeData,
    session: &Arc<Session>,
    cluster_name: &str,
    zid: &str,
) {
    let data: NewVoronoiRequest = deserialize(payload).unwrap();
    //get random point to give to new node

    let mut rng = rand::thread_rng();
    let mut point = data.site;

    //do something here
    //find closest node to new point

    if boot_node_data.cluster.is_empty() {
        //point = (50.0, 50.0);
        boot_node_data
            .cluster
            .insert(data.sender_id.to_string(), point);
        boot_node_data
            .polygon_list
            .insert(data.sender_id.to_string(), vec![]);
    } else {
        //check if a point exist in boot_node.cluster.values that is within X distance of the new point if so precalculate the new point
        point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
        let tolerance = 0.1;
        while point_within_distance(&boot_node_data.cluster, point, tolerance) {
            point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
            print!("IM CHANGING THE SITE! :)");
        }
    }

    // //get site from text file - testing only
    // let sites_path = dirs::document_dir()
    //     .unwrap()
    //     .join("SpaceNet")
    //     .join("logs")
    //     .join(format!("{}.log", "stress_test1"));
    // let file = File::open(sites_path).unwrap();
    // let reader = BufReader::new(file);
    // for (i, line_result) in reader.lines().enumerate() {
    //     let line = line_result.unwrap();
    //     if i == boot_node.draw_count as usize {
    //         let mut parts = line[1..line.len() - 1].split(", ");
    //         let x = parts.next().unwrap().parse::<f64>().unwrap();
    //         let y = parts.next().unwrap().parse::<f64>().unwrap();
    //         point = (x, y);
    //         if boot_node.draw_count == 86 {
    //             point = (x, y);
    //         }
    //         break;
    //     }
    // }

    println!("------------------------------------");
    println!("Giving point {:?}.... to {:?}", point, data.sender_id);
    println!("------------------------------------");

    let (_, land_owner) = closest_point(&boot_node_data.cluster, point);
    println!("OWNER: {}", land_owner);

    //add node to cluster
    boot_node_data
        .cluster
        .insert(data.sender_id.to_string(), point);
    boot_node_data
        .polygon_list
        .insert(data.sender_id.to_string(), vec![]);

    //send land owner its
    let json_message = serialize(&NewNodeResponse {
        new_site: point,
        new_id: data.sender_id,
        sender_id: zid.to_string(),
    })
    .unwrap();

    session
        .put(
            format!("{}/node/{}/owner_request", cluster_name, land_owner),
            json_message,
        )
        .res()
        .unwrap();
}

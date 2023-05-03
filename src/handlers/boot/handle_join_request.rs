use crate::boot_node::BootNode;
use crate::message::{
    DefaultMessage, NeighboursResponse, NewNodeResponse, NewResponse, NewVoronoiRequest,
};
use crate::node::SyncResolve;
use crate::types::{closest_point, point_within_distance, OrderedMapPolygon};
use crate::utils::{draw_voronoi_full, Voronoi};
use bincode::{deserialize, serialize};
use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Handles a join request from a new node. Boot node assigns a point to the new node and states the closest node (`land_owner`) to the new point. Sends the new node the zid of the 'land_owner' node.
/// Adds new node to cluster and polygon list of boot node.
pub fn handle_join_request(payload: &[u8], boot_node: &mut BootNode) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    //get random point to give to new node

    let mut rng = rand::thread_rng();
    let mut point = (-1., -1.);

    //find closest node to new point
    if boot_node.cluster.is_empty() {
        point = (50.0, 50.0);
        boot_node.cluster.insert(data.sender_id.to_string(), point);
        boot_node
            .polygon_list
            .insert(data.sender_id.to_string(), vec![]);
    } else {
        //check if a point exist in boot_node.cluster.values that is within X distance of the new point if so precalculate the new point
        point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
        let tolerance = 0.1;
        while point_within_distance(&boot_node.cluster, point, tolerance) {
            point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
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
    //         if boot_node.draw_count==86{
    //             point = (x, y);
    //         }
    //         break;
    //     }
    // }

    println!("------------------------------------");
    println!("Giving point {:?}.... to {:?}", point, data.sender_id);
    println!("------------------------------------");

    // let (land_owner_site, land_owner) = closest_point(&boot_node.cluster, point);
    // println!("{}", land_owner);

    //correct voronoi
    // boot_node.correct_polygon_list = OrderedMapPolygon::new();

    let diagram = Voronoi::new((data.sender_id.clone(), point), &boot_node.cluster);
    for (i, cell) in diagram.diagram.cells().iter().enumerate() {
        let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
        let site_id = diagram.input.keys().nth(i).unwrap();
        boot_node
            .correct_polygon_list
            .insert(site_id.to_string(), polygon);
    }

    let polygon: Vec<(f64, f64)> = diagram.diagram.cells()[0]
        .points()
        .iter()
        .map(|x| (x.x, x.y))
        .collect();

    //add node to cluster
    boot_node.cluster.insert(data.sender_id.to_string(), point);


    draw_voronoi_full(
        &boot_node.cluster,
        &boot_node.correct_polygon_list,
        format!("{}confirm", boot_node.draw_count).as_str(),
    );

    boot_node
        .polygon_list
        .insert(data.sender_id.to_string(), polygon);

    let new_site_neighbours = diagram.get_neighbours();

    let json_message = serialize(&NewResponse {
        neighbours: new_site_neighbours.clone(),
        new_site: point,
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

    boot_node.expected_counter = new_site_neighbours.len() as i32;

    //send each node in new_site_neighbours the new nodes point
    let message = serialize(&NewVoronoiRequest {
        site: point,
        sender_id: boot_node.zid.clone(),
    })
    .unwrap();
    for neighbour_id in new_site_neighbours.keys() {
        boot_node
            .session
            .put(
                format!(
                    "{}/node/{}/new_voronoi",
                    boot_node.cluster_name, neighbour_id
                ),
                message.clone(),
            )
            .res()
            .unwrap();
    }
}

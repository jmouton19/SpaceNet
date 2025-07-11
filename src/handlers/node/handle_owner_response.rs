use crate::message::{DefaultMessage, ExpectedNodes, NewVoronoiResponse, OwnerResponse};
use crate::node::{NodeData, NodeStatus, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

use std::sync::Arc;
use zenoh::Session;

/// Sets given site and calculates initial voronoi from owner and his neighbours. Then asks for neighbours from neighbours.
pub fn handle_owner_response(
    payload: &[u8],
    node_data: &mut NodeData,
    session: &Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    let data: OwnerResponse = deserialize(payload).unwrap();
    println!(
        "New node at site... {:?} from owner id... {:?}",
        data.new_site, data.sender_id
    );

    node_data.site = data.new_site;
    node_data.neighbours = data.neighbours.clone();
    node_data
        .neighbours
        .insert(data.sender_id.clone(), data.sender_site);

    println!("My neighbours are {:?}", node_data.neighbours);

    //calc initial voronoi
    let diagram = Voronoi::new((zid.to_string(), node_data.site), &node_data.neighbours);
    // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
    //my new visible neighbours
    node_data.neighbours = diagram.get_neighbours();

    //set my k-hop wait
    node_data.expected_counter = node_data.neighbours.len() as i32;

    //ask initial neighbours for their neighbours
    //request neighbours from neighbours and send it to new node

    let message = serialize(&DefaultMessage {
        sender_id: zid.to_string(),
    })
    .unwrap();

    if node_data.neighbours.is_empty() {
        let message = serialize(&ExpectedNodes {
            number: 1,
            sender_id: zid.to_string(),
        })
        .unwrap();
        session
            .put(format!("{}/counter/expected_wait", cluster_name), message)
            .res()
            .unwrap();

        println!("NEW NODE POLYGON DONE!");

        let polygon: Vec<(f64, f64)> =
            vec![(-0.0, 100.0), (-0.0, 0.0), (100.0, -0.0), (100.0, 100.0)];

        node_data.polygon = polygon.clone();

        let message = serialize(&NewVoronoiResponse {
            polygon,
            sender_id: zid.to_string(),
            site: node_data.site,
        })
        .unwrap();
        session
            .put(
                format!("{}/counter/complete", cluster_name),
                message.clone(),
            )
            .res()
            .unwrap();
        session
            .put(format!("{}/sse/event/polygon_add", cluster_name), message)
            .res()
            .unwrap();
        node_data.status = NodeStatus::Online;
    } else {
        for neighbour_id in node_data.neighbours.keys() {
            session
                .put(
                    format!(
                        "{}/node/{}/neighbours_neighbours",
                        cluster_name, neighbour_id
                    ),
                    message.clone(),
                )
                .res()
                .unwrap();
        }
    }
}

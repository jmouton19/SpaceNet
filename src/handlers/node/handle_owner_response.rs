use crate::message::{DefaultMessage, ExpectedNodes, NewVoronoiResponse, OwnerResponse};
use crate::node::{Node, NodeData, NodeStatus, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};
use std::sync::{Arc, MutexGuard};
use zenoh::Session;

/// Sets given site and calculates initial voronoi from owner and his neighbours. Then asks for neighbours from neighbours.
pub fn handle_owner_response(
    payload: &[u8],
    mut node_data: MutexGuard<NodeData>,
    session: Arc<Session>,
) {
    println!("IM NOT BEING USED");
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
    let diagram = Voronoi::new(
        (node_data.zid.clone(), node_data.site),
        &node_data.neighbours,
    );
    // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
    //my new visible neighbours
    node_data.neighbours = diagram.get_neighbours();

    //set my k-hop wait
    node_data.expected_counter = node_data.neighbours.len() as i32;

    //ask initial neighbours for their neighbours
    //request neighbours from neighbours and send it to new node

    let message = serialize(&DefaultMessage {
        sender_id: node_data.zid.clone(),
    })
    .unwrap();

    if node_data.neighbours.is_empty() {
        let message = serialize(&ExpectedNodes {
            number: 1,
            sender_id: node_data.zid.clone(),
        })
        .unwrap();
        session
            .put(
                format!("{}/counter/expected_wait", node_data.cluster_name),
                message,
            )
            .res()
            .unwrap();

        println!("NEW NODE POLYGON DONE!");
        let polygon: Vec<(f64, f64)> = diagram.diagram.cells()[0]
            .points()
            .iter()
            .map(|x| (x.x, x.y))
            .collect();
        node_data.polygon = polygon.clone();
        let message = serialize(&NewVoronoiResponse {
            polygon,
            sender_id: node_data.zid.clone(),
            site: node_data.site,
        })
        .unwrap();
        session
            .put(
                format!("{}/counter/complete", node_data.cluster_name),
                message,
            )
            .res()
            .unwrap();
        node_data.status = NodeStatus::Online;
    } else {
        for neighbour_id in node_data.neighbours.keys() {
            session
                .put(
                    format!(
                        "{}/node/{}/neighbours_neighbours",
                        node_data.cluster_name, neighbour_id
                    ),
                    message.clone(),
                )
                .res()
                .unwrap();
        }
    }
}

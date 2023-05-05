use crate::message::{
    DefaultMessage, ExpectedNodes, NewVoronoiRequest, NewVoronoiResponse, OwnerResponse,
};
use crate::node::{Node, NodeStatus, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

/// Sets site given from boot node and messages land owner to request neighbour list
pub fn handle_owner_response(payload: &[u8], node: &mut Node) {
    let data: OwnerResponse = deserialize(payload).unwrap();
    println!(
        "New node at site... {:?} from owner id... {:?}",
        data.new_site, data.sender_id
    );

    node.site = data.new_site;
    node.neighbours = data.neighbours.clone();
    node.neighbours
        .insert(data.sender_id.clone(), data.sender_site);

    println!("My neighbours are {:?}", node.neighbours);

    //calc initial voronoi
    let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
    // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
    //my new visible neighbours
    node.neighbours = diagram.get_neighbours();

    //set my k-hop wait
    node.expected_counter = node.neighbours.len() as i32;

    //ask initial neighbours for their neighbours
    //request neighbours from neighbours and send it to new node

    let message = serialize(&DefaultMessage {
        sender_id: node.zid.clone(),
    })
    .unwrap();

    if node.neighbours.is_empty() {
        let message = serialize(&ExpectedNodes {
            number: 1,
            sender_id: node.zid.clone(),
        })
        .unwrap();
        node.session
            .put(
                format!("{}/counter/expected_wait", node.cluster_name),
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
        node.polygon = polygon.clone();
        let message = serialize(&NewVoronoiResponse {
            polygon,
            sender_id: node.zid.clone(),
            site: node.site,
        })
        .unwrap();
        node.session
            .put(format!("{}/counter/complete", node.cluster_name), message)
            .res()
            .unwrap();
        node.status = NodeStatus::Online;
    } else {
        for neighbour_id in node.neighbours.keys() {
            node.session
                .put(
                    format!(
                        "{}/node/{}/neighbours_neighbours",
                        node.cluster_name, neighbour_id
                    ),
                    message.clone(),
                )
                .res()
                .unwrap();
        }
    }
}

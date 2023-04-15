use crate::message::{ExpectedNodes, NeighboursResponse, NewVoronoiRequest, NewVoronoiResponse};
use crate::node::{Node, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

pub fn handle_neighbours_neighbours_response(payload: &[u8], node: &mut Node) {
    let data: NeighboursResponse = deserialize(payload).unwrap();
    node.neighbours.extend(data.neighbours);
    node.neighbours.remove(node.zid.as_str());
    node.received_counter += 1;
    println!(
        "Message received from {}....  expecting {} more.",
        data.sender_id,
        node.expected_counter - node.received_counter
    );
    if node.expected_counter == node.received_counter {
        node.received_counter = 0;
        node.expected_counter = -1;

        //tell boot how many to wait for
        let message = serialize(&ExpectedNodes {
            number: node.neighbours.len() as i32 + 1,
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

        //tell all neighbours to calc new voronoi with my new site.
        let message = serialize(&NewVoronoiRequest {
            site: node.site,
            sender_id: node.zid.clone(),
        })
        .unwrap();
        for neighbour_id in node.neighbours.keys() {
            node.session
                .put(
                    format!("{}/node/{}/new_voronoi", node.cluster_name, neighbour_id),
                    message.clone(),
                )
                .res()
                .unwrap();
        }

        //calc my own voronoi with all neighbours.
        let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
        //  draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
        //get my visible neighbours
        node.neighbours = diagram.get_neighbours();

        println!("IM DONE BOOT!");
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
    } //else do nothing
}

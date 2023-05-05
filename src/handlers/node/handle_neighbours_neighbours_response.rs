use crate::message::{
    DefaultMessage, ExpectedNodes, NeighboursResponse, NewVoronoiRequest, NewVoronoiResponse,
};
use crate::node::{Node, NodeStatus, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};
use std::collections::HashSet;

/// Keep asking for neighbours until voronoi edges are stable. Once stable tell my neighbours to recalculate voronoi with my new site.
pub fn handle_neighbours_neighbours_response(payload: &[u8], node: &mut Node) {
    let data: NeighboursResponse = deserialize(payload).unwrap();

    if node.status != NodeStatus::Online {
        println!("NEXT K-HOP NUMBER.....");

        node.k_hop_neighbours.extend(data.neighbours);
        //node.neighbours.extend(data.neighbours);
        node.received_counter += 1;
        println!(
            "Message received from {}....  expecting {} more.",
            data.sender_id,
            node.expected_counter - node.received_counter
        );
        if node.expected_counter == node.received_counter {
            node.received_counter = 0;
            node.expected_counter = -1;

            //calc new voronoi then check if neigh list changed
            let mut temp = node.neighbours.clone();
            temp.extend(node.k_hop_neighbours.clone());
            let diagram = Voronoi::new((node.zid.clone(), node.site), &temp);
            //  draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
            //get my visible neighbours

            let old_neighbours_keys: HashSet<_> = node.neighbours.keys().cloned().collect();
            let new_neighbours = diagram.get_neighbours();
            let new_neighbours_keys: HashSet<_> = new_neighbours.keys().cloned().collect();

            let added: Vec<_> = new_neighbours_keys
                .difference(&old_neighbours_keys)
                .cloned()
                .collect();

            if !added.is_empty() {
                node.status = NodeStatus::Joining;
                node.neighbours = new_neighbours;
                //ask new neighbour list for their neighbours
                node.expected_counter = added.len() as i32;
                let message = serialize(&DefaultMessage {
                    sender_id: node.zid.clone(),
                })
                .unwrap();
                for neighbour_id in added {
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
            } else {
                node.k_hop_neighbours.clear();
                //neighbor unchanged so finalize
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
            }
        } //else do nothing
    }
}

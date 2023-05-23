use crate::message::{
    DefaultMessage, ExpectedNodes, NeighboursResponse, NewVoronoiRequest, NewVoronoiResponse,
};
use crate::node::{NodeData, NodeStatus, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

use std::collections::HashSet;
use std::sync::Arc;
use zenoh::Session;

/// Keep asking for neighbours until voronoi edges are stable. Once stable tell my neighbours to recalculate voronoi with my new site.
pub fn handle_neighbours_neighbours_response(
    payload: &[u8],
    node_data: &mut NodeData,
    session: Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    let data: NeighboursResponse = deserialize(payload).unwrap();

    if node_data.status != NodeStatus::Online {
        println!("NEXT K-HOP NUMBER.....");

        node_data.k_hop_neighbours.extend(data.neighbours);
        //node.neighbours.extend(data.neighbours);
        node_data.received_counter += 1;
        println!(
            "Message received from {}....  expecting {} more.",
            data.sender_id,
            node_data.expected_counter - node_data.received_counter
        );
        if node_data.expected_counter == node_data.received_counter {
            node_data.received_counter = 0;
            node_data.expected_counter = -1;

            //calc new voronoi then check if neigh list changed
            let mut temp = node_data.neighbours.clone();
            temp.extend(node_data.k_hop_neighbours.clone());
            let diagram = Voronoi::new((zid.to_string(), node_data.site), &temp);
            //  draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
            //get my visible neighbours

            let old_neighbours_keys: HashSet<_> = node_data.neighbours.keys().cloned().collect();
            let new_neighbours = diagram.get_neighbours();
            let new_neighbours_keys: HashSet<_> = new_neighbours.keys().cloned().collect();

            let added: Vec<_> = new_neighbours_keys
                .difference(&old_neighbours_keys)
                .cloned()
                .collect();

            if !added.is_empty() {
                node_data.status = NodeStatus::Joining;
                node_data.neighbours = new_neighbours;
                //ask new neighbour list for their neighbours
                node_data.expected_counter = added.len() as i32;
                let message = serialize(&DefaultMessage {
                    sender_id: zid.to_string(),
                })
                .unwrap();
                for neighbour_id in added {
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
            } else {
                node_data.k_hop_neighbours.clear();
                //neighbor unchanged so finalize
                //tell boot how many to wait for
                let message = serialize(&ExpectedNodes {
                    number: node_data.neighbours.len() as i32 + 1,
                    sender_id: zid.to_string(),
                })
                .unwrap();
                session
                    .put(format!("{}/counter/expected_wait", cluster_name), message)
                    .res()
                    .unwrap();

                //tell all neighbours to calc new voronoi with my new site.
                let message = serialize(&NewVoronoiRequest {
                    site: node_data.site,
                    sender_id: zid.to_string(),
                })
                .unwrap();
                for neighbour_id in node_data.neighbours.keys() {
                    session
                        .put(
                            format!("{}/node/{}/new_voronoi", cluster_name, neighbour_id),
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
                node_data.polygon = polygon.clone();
                let message = serialize(&NewVoronoiResponse {
                    polygon,
                    sender_id: zid.to_string(),
                    site: node_data.site,
                })
                .unwrap();
                session
                    .put(format!("{}/counter/complete", cluster_name), message)
                    .res()
                    .unwrap();
                node_data.status = NodeStatus::Online;
            }
        } //else do nothing
    }
}

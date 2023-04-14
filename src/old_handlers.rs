use crate::message::*;
use crate::node::*;
use crate::types::{closest_point, OrderedMapPairs, OrderedMapPolygon};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};
use rand::Rng;
use zenoh::prelude::Sample;

/// Callback function to handle messages on topics for a node
pub fn node_callback(sample: Sample, node: &mut Node) {
    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
    println!("Received message on topic... {:?}", topic);
    let payload = sample.value.payload.get_zslice(0).unwrap();

    match topic {
        "new_reply" => {
            let data: NewNodeResponse = deserialize(payload.as_ref()).unwrap();
            println!(
                "New point.... {:?} owner... {:?}",
                data.new_site, data.land_owner
            );

            //set site to given site
            node.site = data.new_site;

            node.neighbours
                .insert(data.land_owner.clone(), data.land_owner_site);
            //request neighbour list from land owner
            let message = serialize(&NewVoronoiRequest {
                sender_id: node.zid.clone(),
                site: node.site,
            })
            .unwrap();
            node.session
                .put(
                    format!("{}/node/{}/new_neighbours", node.cluster, data.land_owner),
                    message,
                )
                .res()
                .unwrap();
        }

        "new_neighbours" => {
            let data: NewVoronoiRequest = deserialize(payload.as_ref()).unwrap();
            println!(
                "New point at site... {:?} from... {:?}",
                data.site, data.sender_id
            );

            let neigh_len = node.neighbours.len() as i32;

            //tell new node how many to wait for
            let message = serialize(&ExpectedNodes {
                number: neigh_len + 1,
                sender_id: node.zid.clone(),
            })
            .unwrap();
            node.session
                .put(
                    format!(
                        "{}/node/{}/neighbours_expected",
                        node.cluster, data.sender_id
                    ),
                    message,
                )
                .res()
                .unwrap();

            //send my neighbors
            let message = serialize(&NeighboursResponse {
                sender_id: node.zid.clone(),
                neighbours: node.neighbours.clone(),
            })
            .unwrap();
            node.session
                .put(
                    format!(
                        "{}/node/{}/neighbours_neighbours_reply",
                        node.cluster, data.sender_id
                    ),
                    message,
                )
                .res()
                .unwrap();

            //request neighbours from neighbours and send it to new node
            let message = serialize(&NeighboursNeighboursRequest {
                new_zid: data.sender_id,
                sender_id: node.zid.clone(),
            })
            .unwrap();
            for neighbour_id in node.neighbours.keys() {
                node.session
                    .put(
                        format!(
                            "{}/node/{}/neighbours_neighbours",
                            node.cluster, neighbour_id
                        ),
                        message.clone(),
                    )
                    .res()
                    .unwrap();
            }
        }
        "neighbours_expected" => {
            let data: ExpectedNodes = deserialize(payload.as_ref()).unwrap();
            node.expected_counter = data.number;
            println!("Im expecting {:?} neighbor responses...", data.number);
        }

        "neighbours_neighbours" => {
            let data: NeighboursNeighboursRequest = deserialize(payload.as_ref()).unwrap();
            //send list of neighbours back to new node
            let message = serialize(&NeighboursResponse {
                sender_id: node.zid.clone(),
                neighbours: node.neighbours.clone(),
            })
            .unwrap();
            node.session
                .put(
                    format!(
                        "{}/node/{}/neighbours_neighbours_reply",
                        node.cluster, data.new_zid
                    ),
                    message,
                )
                .res()
                .unwrap();
        }
        "leave_neighbours_neighbours" => {
            let data: DefaultMessage = deserialize(payload.as_ref()).unwrap();
            //send list of neighbours back to leaver
            let message = serialize(&NeighboursResponse {
                sender_id: node.zid.clone(),
                neighbours: node.neighbours.clone(),
            })
            .unwrap();
            node.session
                .put(
                    format!(
                        "{}/node/{}/Leave_neighbours_neighbours_reply",
                        node.cluster, data.sender_id
                    ),
                    message,
                )
                .res()
                .unwrap();
        }

        "neighbours_neighbours_reply" => {
            let data: NeighboursResponse = deserialize(payload.as_ref()).unwrap();
            node.neighbours.extend(data.neighbours);
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
                    .put(format!("{}/counter/expected_wait", node.cluster), message)
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
                            format!("{}/node/{}/new_voronoi", node.cluster, neighbour_id),
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
                let polygon = diagram.diagram.cells()[0]
                    .points()
                    .iter()
                    .map(|x| (x.x, x.y))
                    .collect();
                let message = serialize(&NewVoronoiResponse {
                    polygon,
                    sender_id: node.zid.clone(),
                })
                .unwrap();
                node.session
                    .put(format!("{}/counter/complete", node.cluster), message)
                    .res()
                    .unwrap();
            } //else do nothing
        }
        "Leave_neighbours_neighbours_reply" => {
            let data: NeighboursResponse = deserialize(payload.as_ref()).unwrap();
            node.neighbours.extend(data.neighbours);
            node.received_counter += 1;
            println!(
                "Message received from {}....  expecting {} more.",
                data.sender_id,
                node.expected_counter - node.received_counter
            );
            if node.expected_counter == node.received_counter {
                node.neighbours.remove(node.zid.as_str());
                node.received_counter = 0;
                node.expected_counter = -1;

                //tell boot how many to wait for
                //+1 if wait for node to say its left
                let message = serialize(&ExpectedNodes {
                    number: node.neighbours.len() as i32,
                    sender_id: node.zid.clone(),
                })
                .unwrap();
                node.session
                    .put(format!("{}/counter/expected_wait", node.cluster), message)
                    .res()
                    .unwrap();

                //tell all neighbours to calc new voronoi without my site.
                let message = serialize(&NeighboursResponse {
                    neighbours: node.neighbours.clone(),
                    sender_id: node.zid.clone(),
                })
                .unwrap();
                for neighbour_id in node.neighbours.keys() {
                    node.session
                        .put(
                            format!("{}/node/{}/leave_voronoi", node.cluster, neighbour_id),
                            message.clone(),
                        )
                        .res()
                        .unwrap();
                }

                //drop node instance
                println!("IM SHUTTING DOWN BOOT!");
                // let message = serialize(&DefaultMessage{
                // sender_id:node.zid.clone()});
                // node.session.put("counter/leaving", message.clone()).res().unwrap();
                node.running = false;
                let _ = node;
            } //else do nothing
        }
        "new_voronoi" => {
            let data: NewVoronoiRequest = deserialize(payload.as_ref()).unwrap();
            println!("Recalculating my voronoi with site... {:?}", data.site);

            //recalculate own voronoi
            node.neighbours
                .insert(data.sender_id.to_string(), data.site);
            let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
            // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            //my new visible neighbours
            node.neighbours = diagram.get_neighbours();
            println!("IM DONE BOOT!");
            let polygon = diagram.diagram.cells()[0]
                .points()
                .iter()
                .map(|x| (x.x, x.y))
                .collect();
            let message = serialize(&NewVoronoiResponse {
                polygon,
                sender_id: node.zid.clone(),
            })
            .unwrap();
            node.session
                .put(format!("{}/counter/complete", node.cluster), message)
                .res()
                .unwrap();
        }
        "leave_voronoi" => {
            let data: NeighboursResponse = deserialize(payload.as_ref()).unwrap();
            println!(
                "Recalculating my voronoi without site... {:?}",
                data.sender_id
            );
            //and leavers neighbours....

            //recalculate own voronoi
            node.neighbours.remove(data.sender_id.as_str());
            node.neighbours.extend(data.neighbours);
            let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
            // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            //my new visible neighbours
            node.neighbours = diagram.get_neighbours();
            println!("IM DONE BOOT!");
            let polygon = diagram.diagram.cells()[0]
                .points()
                .iter()
                .map(|x| (x.x, x.y))
                .collect();
            let message = serialize(&NewVoronoiResponse {
                polygon,
                sender_id: node.zid.clone(),
            })
            .unwrap();
            node.session
                .put(format!("{}/counter/complete", node.cluster), message)
                .res()
                .unwrap();
        }
        "leave_reply" => {
            //tell me how many to wait for
            node.expected_counter = node.neighbours.len() as i32;
            println!(
                "Expecting {} replies... before i leave",
                node.expected_counter
            );

            //get FULL neighbour list
            //request neighbours from neighbours and send it back to me
            let message = serialize(&DefaultMessage {
                sender_id: node.zid.clone(),
            })
            .unwrap();
            for neighbour_id in node.neighbours.keys() {
                node.session
                    .put(
                        format!(
                            "{}/node/{}/leave_neighbours_neighbours",
                            node.cluster, neighbour_id
                        ),
                        message.clone(),
                    )
                    .res()
                    .unwrap();
            }
        }
        _ => println!("UNKNOWN NODE TOPIC"),
    }
}

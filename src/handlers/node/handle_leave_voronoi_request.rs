use crate::message::{NeighboursResponse, NewVoronoiResponse};
use crate::node::{Node, NodeData, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};
use std::sync::{Arc, MutexGuard};
use zenoh::Session;

/// Calculates new voronoi without leavers site but with his neighbours and sends new polygon to boot node.
pub fn handle_leave_voronoi_request(
    payload: &[u8],
    mut node_data: MutexGuard<NodeData>,
    session: Arc<Session>,
) {
    let data: NeighboursResponse = deserialize(payload).unwrap();
    println!(
        "Recalculating my voronoi without site... {:?}",
        data.sender_id
    );
    //and leavers neighbours....

    //recalculate own voronoi
    node_data.neighbours.remove(data.sender_id.as_str());
    node_data.neighbours.extend(data.neighbours);
    let zid = node_data.zid.clone();
    node_data.neighbours.remove(zid.as_str());
    if node_data.neighbours.is_empty() {
        node_data.site = (50.0, 50.0);
    }

    let diagram = Voronoi::new(
        (node_data.zid.clone(), node_data.site),
        &node_data.neighbours,
    );
    // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
    //my new visible neighbours
    node_data.neighbours = diagram.get_neighbours();
    println!("IM DONE BOOT!");
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
}

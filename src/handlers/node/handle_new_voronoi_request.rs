use crate::message::{NewVoronoiRequest, NewVoronoiResponse};
use crate::node::{NodeData, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

use std::sync::Arc;
use zenoh::Session;

/// Recalculates voronoi with new site and sends new polygon to boot node
pub fn handle_new_voronoi_request(
    payload: &[u8],
    node_data: &mut NodeData,
    session: &Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    let data: NewVoronoiRequest = deserialize(payload).unwrap();
    println!("Recalculating my voronoi with site... {:?}", data.site);

    //recalculate own voronoi
    node_data
        .neighbours
        .insert(data.sender_id.to_string(), data.site);
    let diagram = Voronoi::new((zid.to_string(), node_data.site), &node_data.neighbours);
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
        .put(
            format!("{}/sse/event/polygon_update", cluster_name),
            message,
        )
        .res()
        .unwrap();
}

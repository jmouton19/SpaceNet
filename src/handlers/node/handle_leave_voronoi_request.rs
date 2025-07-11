use crate::message::{NeighboursResponse, NewVoronoiResponse};
use crate::node::{NodeData, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

use std::sync::Arc;
use zenoh::Session;

/// Calculates new voronoi without leavers site but with his neighbours and sends new polygon to boot node.
pub fn handle_leave_voronoi_request(
    payload: &[u8],
    node_data: &mut NodeData,
    session: &Arc<Session>,
    zid: &str,
    cluster_name: &str,
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

    {
        let zid = zid.to_string();
        node_data.neighbours.remove(zid.as_str());
    }

    println!("IM DONE BOOT!");
    let polygon: Vec<(f64, f64)>;
    if node_data.neighbours.is_empty() {
        polygon = vec![(-0.0, 100.0), (-0.0, 0.0), (100.0, -0.0), (100., 100.0)];
    } else {
        let diagram = Voronoi::new((zid.to_string(), node_data.site), &node_data.neighbours);
        // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
        //my new visible neighbours
        node_data.neighbours = diagram.get_neighbours();
        polygon = diagram.diagram.cells()[0]
            .points()
            .iter()
            .map(|x| (x.x, x.y))
            .collect();
    }

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

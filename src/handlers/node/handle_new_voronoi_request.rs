use crate::message::{NewVoronoiRequest, NewVoronoiResponse};
use crate::node::{Node, SyncResolve};
use crate::utils::Voronoi;
use bincode::{deserialize, serialize};

/// Recalculates voronoi with new site and sends new polygon to boot node
pub fn handle_new_voronoi_request(payload: &[u8], node: &mut Node) {
    let data: NewVoronoiRequest = deserialize(payload).unwrap();
    println!("Recalculating my voronoi with site... {:?}", data.site);

    //recalculate own voronoi
    node.neighbours
        .insert(data.sender_id.to_string(), data.site);
    let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
    // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
    //my new visible neighbours
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
}

use crate::boot_node::BootNodeData;
use crate::types::OrderedMapPolygon;
use crate::utils::{draw_voronoi_full, Voronoi};

pub fn generate_output(boot_node_data: &mut BootNodeData) {
    //redraw distributed voronoi
    draw_voronoi_full(
        &boot_node_data.cluster,
        &boot_node_data.polygon_list,
        format!("{}voronoi", boot_node_data.draw_count).as_str(),
    );
    if boot_node_data.centralized_voronoi {
        //correct voronoi
        let mut hash_map = boot_node_data.cluster.clone();
        boot_node_data.correct_polygon_list = OrderedMapPolygon::new();
        let (first_zid, first_site) = hash_map
            .iter()
            .next()
            .map(|(k, v)| (k.clone(), *v))
            .unwrap();
        hash_map.swap_remove_index(0);
        let diagram = Voronoi::new((first_zid, first_site), &hash_map);
        for (i, cell) in diagram.diagram.cells().iter().enumerate() {
            let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
            let site_id = diagram.input.keys().nth(i).unwrap();
            boot_node_data
                .correct_polygon_list
                .insert(site_id.to_string(), polygon);
        }
        draw_voronoi_full(
            &boot_node_data.cluster,
            &boot_node_data.correct_polygon_list,
            format!("{}confirm", boot_node_data.draw_count).as_str(),
        );
    }
    boot_node_data.draw_count += 1;
}

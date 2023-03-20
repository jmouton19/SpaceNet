use rand::Rng;
use zenoh::prelude::Sample;
use crate::node::*;
use crate::message::*;
use crate::utils::{draw_voronoi, Voronoi};

pub fn node_callback(sample: Sample, node: &mut Node) {
    let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);

    match topic {
        "new_reply" => {
            let data: NewNodeResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("New point.... {:?} owner... {:?}", data.site, data.land_owner);

            //set site to given site
            node.site=data.site;

            //add land owner to neighbours
            node.neighbours.push_pair(data.land_owner_site,data.land_owner);

            //request neighbour list from land owner
            let message = json!(NeighboursRequest{
            sender_id:node.session.zid(),
            site:node.site});
            node.session.put(format!("node/{}/new_neighbours",data.land_owner), message).res().unwrap();
        },

        "new_neighbours" =>{
            let data: NeighboursRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("New point at site... {:?} from... {:?}", data.site, data.sender_id);


            //request neighbours from neighbours and send it to new node




            //send list of neighbour back to new node
            let message = json!(NeighboursResponse{
            sender_id:node.session.zid(),
            neighbours:node.neighbours.clone()});
            node.session.put(format!("node/{}/new_neighbours_reply",data.sender_id), message.clone()).res().unwrap();


            //tell boot how many nodes to wait for (me,new,my neighbours)
            let message = json!(ExpectedNodes{
            sender_id:node.session.zid(),
            number:node.neighbours.sites.len()+2,});
            node.session.put(format!("counter/expected_wait"), message.clone()).res().unwrap();


            //tell each neighbour to recalculate its voronoi
            let message = json!(NewVoronoiRequest{
            new_zid:data.sender_id,
            new_site:data.site,
            sender_id:node.session.zid()});
            for neighbour_id in &node.neighbours.ids{
                node.session.put(format!("node/{}/new_voronoi",neighbour_id), message.clone()).res().unwrap();
            };

            //recalculate own voronoi
            node.neighbours.push_pair(data.site,data.sender_id);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            node.neighbours=diagram.get_neighbours();

            println!("IM DONE BOOT!");
            let polygon=diagram.diagram.cells()[0].points().iter().map(|x|(x.x, x.y)).collect();
            let message = json!(NewVoronoiResponse{
            polygon:polygon,
            sender_id:node.session.zid()});
            node.session.put("counter/complete", message.clone()).res().unwrap();

        },

        "neighbours_neighbours" =>{

        },

        "new_neighbours_reply" =>{
            let data: NeighboursResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Neighbour list received... {:?} from... {:?}", data.neighbours, data.sender_id);

            //calculate cell of new node
            node.push_pair_list(data.neighbours);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            //get new neighbours
            node.neighbours=diagram.get_neighbours();

            println!("IM DONE BOOT!");
            let polygon=diagram.diagram.cells()[0].points().iter().map(|x|(x.x, x.y)).collect();
            let message = json!(NewVoronoiResponse{
            polygon:polygon,
            sender_id:node.session.zid()});
            node.session.put("counter/complete", message.clone()).res().unwrap();

        },

        "new_voronoi" =>{
            let data: NewVoronoiRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Recalculating my voronoi with site... {:?}", data.new_site);

            //recalculate own voronoi
            node.neighbours.push_pair(data.new_site,data.new_zid);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            node.neighbours=diagram.get_neighbours();

            println!("IM DONE BOOT!");
            let polygon=diagram.diagram.cells()[0].points().iter().map(|x|(x.x, x.y)).collect();
            let message = json!(NewVoronoiResponse{
            polygon:polygon,
            sender_id:node.session.zid()});
            node.session.put("counter/complete", message.clone()).res().unwrap();

        },
        _ => println!("UNKNOWN NODE TOPIC"),
    }

}


pub fn boot_callback(sample:Sample, node: &mut Node, polygon_list: &mut Vec<Vec<(f64, f64)>>, cluster: &mut SiteIdPairs){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();

            //get random point to give to new node
            let mut rng = rand::thread_rng();
            let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple
            while cluster.sites.contains(&point) {
                point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // if tuple is in exclude list, generate a new one
            }

            println!("------------------------------------");
            println!("Giving point {:?}.... to {:?}",point,data.sender_id);
            println!("------------------------------------");

            //find closest node to new point
            let index=cluster.closest_point(point);
            let land_owner =cluster.ids[index];

            //add node to cluster
            cluster.push_pair(point,data.sender_id);
            polygon_list.push(vec!());


            let json_message = json!(NewNodeResponse{
                site:point,
                land_owner:land_owner,
                land_owner_site:cluster.sites[index],
                sender_id:node.session.zid()
            });

            let _ = node.session.put(format!("node/{}/new_reply",data.sender_id), json_message).res();
        }
        _=> println!("UNKNOWN BOOT TOPIC"),

    }
}
pub fn counter_callback(sample:Sample, expected_counter:&mut i32, counter: &mut i32, polygon_list: &mut Vec<Vec<(f64, f64)>>, cluster: &mut SiteIdPairs){
    let topic=sample.key_expr.split('/').nth(1).unwrap_or("");
    println!("Topic... {:?}",topic);
    match topic {
        "expected_wait"=>{
            let data: ExpectedNodes = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Im waiting for {} nodes to reply...",data.number);
            *expected_counter=data.number as i32;
        },
        "complete"=>{
            *counter+=1;
            let data: NewVoronoiResponse = serde_json::from_str(&sample.value.to_string()).unwrap();

            if let Some(index) = cluster.ids.iter().position(|&id| id == data.sender_id) {
                polygon_list[index]=data.polygon;
            };
        },
        _=> println!("UNKNOWN COUNTER TOPIC"),

    }
}
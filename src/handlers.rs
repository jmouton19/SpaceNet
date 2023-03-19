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
            println!("Given point.... {:?} owner... {:?}", data.site, data.land_owner);

            //set site to given site
            node.site=data.site;

            //add land owner to neighbours
            node.neighbours.push_pair(data.land_owner_site,data.land_owner);

            //request neighbour list from land owner
            let message = json!(NeighboursRequest{
            value:"Hello, imma join you".to_string(),
            sender_id:node.session.zid(),
            site:node.site});
            node.session.put(format!("node/{}/new_neighbours",data.land_owner), message).res().unwrap();
        },

        "new_neighbours" =>{
            let data: NeighboursRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("New point at site... {:?} from... {:?}", data.site, data.sender_id);

            //send list of neighbour back to new node
            let message = json!(NeighboursResponse{
            value:"Here's a list of my neighbours".to_string(),
            sender_id:node.session.zid(),
            neighbours:node.neighbours.clone()});
            node.session.put(format!("node/{}/new_neighbours_reply",data.sender_id), message.clone()).res().unwrap();



            let message = json!(ExpectedNodes{
            value:"This is how many node you gotta wait for".to_string(),
            sender_id:node.session.zid(),
            number:node.neighbours.sites.len()+2,});

            //tell boot how many nodes to wait for (me,new,my neighbours)
            node.session.put(format!("node/boot/expected_wait"), message.clone()).res().unwrap();

            //tell each neighbour to recalculate its voronoi
            let message = json!(NewVoronoiRequest{
            value:"This is the new node".to_string(),
            new_zid:data.sender_id,
            new_site:data.site,
            sender_id:node.session.zid()});
            for neighbour_id in &node.neighbours.ids{
                node.session.put(format!("node/{}/new_voronoi",neighbour_id), message.clone()).res().unwrap();
            };

            //recalculate own voronoi
            node.neighbours.push_pair(data.site,data.sender_id);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str(),false);
            node.neighbours=diagram.get_neighbours();
            println!("IM DONE BOOT!");

        },

        "new_neighbours_reply" =>{
            let data: NeighboursResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Neighbour list received... {:?} from... {:?}", data.neighbours, data.sender_id);

            //calculate cell of new node
            node.push_pair_list(data.neighbours);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str(),false);
            //get new neighbours
            node.neighbours=diagram.get_neighbours();
            println!("IM DONE BOOT!");

        },

        "new_voronoi" =>{

            let data: NewVoronoiRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Recalculating my voronoi with site... {:?}", data.new_site);

            //recalculate own voronoi
            node.neighbours.push_pair(data.new_site,data.new_zid);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str(),false);
            node.neighbours=diagram.get_neighbours();

            println!("IM DONE BOOT!");
            let message = json!(NewVoronoiResponse{
            value:"Im done calculating my voronoi".to_string(),
            success:true,
            sender_id:node.session.zid()});
            node.session.put("node/boot/complete", message.clone()).res().unwrap();

        },
        _ => println!("What topic is that lmao"),
    }

}


pub fn boot_callback(sample:Sample, node: &mut Node, cluster: &mut SiteIdPairs, reply_counter: &mut usize){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("{}.... from {}",data.value,data.sender_id);

            //get random point to give to new node
            let mut rng = rand::thread_rng();
            let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple
            while cluster.sites.contains(&point) {
                point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // if tuple is in exclude list, generate a new one
            }


            //find closest node to new point
            let index=cluster.closest_point(point);
            let land_owner =cluster.ids[index];

            //add node to cluster
            cluster.push_pair(point,data.sender_id);

            let json_message = json!(NewNodeResponse{
                value:"New node acknowledged... ".to_string(),
                site:point,
                land_owner:land_owner,
                land_owner_site:cluster.sites[index],
                sender_id:node.session.zid()
            });

            let _ = node.session.put(format!("node/{}/new_reply",data.sender_id), json_message).res();
        }
        "expected_wait"=>{
            let data: ExpectedNodes = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Im waiting for {} nodes to reply...",data.number);
            *reply_counter=data.number;
        },
        _=> println!("what topic is that lmao?"),

    }
}
use rand::Rng;
use zenoh::prelude::Sample;
use crate::node::*;
use crate::message::*;

pub fn node_callback(sample: Sample, node: &mut Node) {
    let topic = sample.key_expr.split('/').nth(2).unwrap_or("");

    match topic {
        "new" => {
            let data: NewNodeResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Given point.... {:?} owner... {:?}", data.site, data.land_owner);
            node.site=data.site;

            let message = json!(NeighboursRequest{
        value:"Hello, imma join you".to_string(),
        sender_id:node.session.zid(),
            site:node.site});
            //message boot node
            node.session.put(format!("node/{}/new/neighbours",data.land_owner), message).res().unwrap();
        },
        "new/neighbours" =>{
            //send list of neighbour back
            node.session.put(format!("node/{}/new/voronoi",neighbour), message.clone()).res().unwrap();


            //tell each neighbour to recalculate his voronoi
            let message = json!(NeighboursRequest{
            value:"This is the new node".to_string(),
            sender_id:node.session.zid(),
            site:node.site});
            for neighbour in &node.neighbours.ids{
                node.session.put(format!("node/{}/new/voronoi",neighbour), message.clone()).res().unwrap();
            };


            let message = json!(NeighboursResponse{
        value:"Hello im new".to_string(),
        neighbours:node.neighbours,
           sender_id:node.session.zid()});

        },
        "new/voronoi" =>{

        }
        _ => println!("What topic is that lmao"),
    }
}


pub fn boot_callback(sample:Sample, node: &mut Node, cluster: &mut SiteIdPairs){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("{}.... from {}",data.value,data.sender_id);


            let mut rng = rand::thread_rng();
            let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple
            while cluster.sites.contains(&point) {
                point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // if tuple is in exclude list, generate a new one
            }

            cluster.push_pair(point,data.sender_id);

            let index=cluster.closest_point(point);
            let land_owner =cluster.ids[index];
            println!("{:?}",land_owner);

            let json_message = json!(NewNodeResponse{
                value:"New node acknowledged... ".to_string(),
                site:point,
                land_owner:land_owner,
                sender_id:node.session.zid()
            });

            let _ = node.session.put(format!("node/{}/new",data.sender_id), json_message).res();
        }
        _=> println!("what topic is that lmao?"),

    }
}
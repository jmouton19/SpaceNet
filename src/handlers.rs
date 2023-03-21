use rand::Rng;
use zenoh::prelude::Sample;
use crate::node::*;
use crate::message::*;
use crate::utils::{draw_voronoi, Voronoi};

pub fn node_callback(sample: Sample, node: &mut Node, expected_counter: &mut i32, counter: &mut i32) {
    let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);

    match topic {
        "new_reply" => {

            let data: NewNodeResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("New point.... {:?} owner... {:?}", data.site, data.land_owner);

            //set site to given site
            node.site=data.site;

            //add land owner to neighbours rather do this later
            //node.neighbours.push_pair(data.land_owner_site,data.land_owner.to_string());


            //request neighbour list from land owner
            let message = json!(NeighboursRequest{
            sender_id:node.session.zid().to_string(),
            new_site:node.site});//not needed...? can get later -NB
            node.session.put(format!("node/{}/new_neighbours",data.land_owner), message).res().unwrap();
        },

        "new_neighbours" =>{
            let data: NeighboursRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("New point at site... {:?} from... {:?}", data.new_site, data.sender_id);

            let neigh_len=node.neighbours.sites.len() as i32;
            println!("{:?}",node.neighbours);


            //get new message type?
            //if have no neighbour just voronoi.
            if neigh_len==0{
                //tell boot node wait 2
                let message = json!(ExpectedNodes{
                number:2,
                sender_id:node.zid.clone()});
                node.session.put("counter/expected_wait", message.clone()).res().unwrap();

                //calc my voronoi
                node.neighbours.sites.insert(data.sender_id.to_string(),data.new_site);
                let diagram = Voronoi::new(node.site, &node.neighbours);
               // draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());


                //get my visible neighbours
                node.neighbours = diagram.get_neighbours();
                println!("NEW NEIGHTBORs {:?}",node.neighbours);
                println!("IM DONE BOOT!");
                let polygon = diagram.diagram.cells()[0].points().iter().map(|x| (x.x, x.y)).collect();
                let message = json!(NewVoronoiResponse{
                polygon:polygon,
                sender_id:node.zid.clone()});
                node.session.put("counter/complete", message.clone()).res().unwrap();


                //tell new site to make voronoi
                let message = json!(NoNeighbours{
                sender_id:node.zid.clone(),
                site:node.site});
                node.session.put(format!("node/{}/no_neighbours",data.sender_id), message.clone()).res().unwrap();



            }else{
                //tell new node how many to wait for
                let message = json!(ExpectedNodes{
                number:neigh_len+1,
                sender_id:node.zid.clone()});
                node.session.put(format!("node/{}/neighbours_expected",data.sender_id), message.clone()).res().unwrap();

                //send my neighbors
                let message = json!(NeighboursResponse{
                sender_id:node.zid.clone(),
                neighbours:node.neighbours.clone()});
                node.session.put(format!("node/{}/neighbours_neighbours_reply",data.sender_id), message.clone()).res().unwrap();

            //request neighbours from neighbours and send it to new node
                let message = json!(NeighboursNeighboursRequest{
               new_zid:data.sender_id,
               sender_id:node.zid.clone()});
                for neighbour_id in node.neighbours.sites.keys(){
                    node.session.put(format!("node/{}/neighbours_neighbours",neighbour_id), message.clone()).res().unwrap();
                };
            }
        },
        "no_neighbours" =>{
            let data: NoNeighbours = serde_json::from_str(&sample.value.to_string()).unwrap();

            node.neighbours.sites.insert(data.sender_id.to_string(),data.site);
            let diagram = Voronoi::new(node.site, &node.neighbours);
           // draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
            //get my visible neighbours
            node.neighbours = diagram.get_neighbours();
            println!("NEW NEIGHTBORs {:?}",node.neighbours);

            println!("IM DONE BOOT!");
            let polygon = diagram.diagram.cells()[0].points().iter().map(|x| (x.x, x.y)).collect();
            let message = json!(NewVoronoiResponse{
                polygon:polygon,
                sender_id:node.zid.clone()});
            node.session.put("counter/complete", message.clone()).res().unwrap();

        },
        "neighbours_expected" =>{
            let data: ExpectedNodes = serde_json::from_str(&sample.value.to_string()).unwrap();
            *expected_counter=data.number as i32;
            println!("Im expecting {expected_counter} neighbor responses...");
        },

        "neighbours_neighbours" =>{
            let data: NeighboursNeighboursRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            //send list of neighbours back to new node
            let message = json!(NeighboursResponse{
            sender_id:node.zid.clone(),
            neighbours:node.neighbours.clone()});
            node.session.put(format!("node/{}/neighbours_neighbours_reply",data.new_zid), message.clone()).res().unwrap();

        },

        "neighbours_neighbours_reply" =>{
            let data: NeighboursResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            node.neighbours.sites.extend(data.neighbours.sites);
            *counter+=1;
        println!("ive got ur kids {} from {} expecting {}",counter,data.sender_id,expected_counter);
        if expected_counter==counter {
            *counter=0;
            *expected_counter=-1;

            //tell boot how many to wait for
            let message = json!(ExpectedNodes{
                number:node.neighbours.sites.len() as i32 +1,
                sender_id:node.zid.clone()});
                node.session.put("counter/expected_wait", message.clone()).res().unwrap();


                //tell all neighbours to calc new voronoi with my new site.
                let message = json!(NewVoronoiRequest{
                new_site:node.site,
                sender_id:node.zid.clone()});
                for neighbour_id in node.neighbours.sites.keys() {
                    node.session.put(format!("node/{}/new_voronoi", neighbour_id), message.clone()).res().unwrap();
                };


                //calc my own voronoi with all neighbours.
                let diagram = Voronoi::new(node.site, &node.neighbours);
              //  draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
                //get my visible neighbours
                node.neighbours = diagram.get_neighbours();

                println!("IM DONE BOOT!");
                let polygon = diagram.diagram.cells()[0].points().iter().map(|x| (x.x, x.y)).collect();
                let message = json!(NewVoronoiResponse{
                polygon:polygon,
                sender_id:node.zid.clone()});
                node.session.put("counter/complete", message.clone()).res().unwrap();
            }//else do nothing
        },
        "new_voronoi" =>{
            let data: NewVoronoiRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Recalculating my voronoi with site... {:?}", data.new_site);


            //recalculate own voronoi
            node.neighbours.sites.insert(data.sender_id.to_string(),data.new_site);
            println!("and neighbours {:?}", node.neighbours);

            let diagram = Voronoi::new(node.site,&node.neighbours);
           // draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
            //my new visible neighbours
            node.neighbours=diagram.get_neighbours();
            println!("IM DONE BOOT!");
            let polygon=diagram.diagram.cells()[0].points().iter().map(|x|(x.x, x.y)).collect();
            let message = json!(NewVoronoiResponse{
            polygon:polygon,
            sender_id:node.zid.clone()});
            node.session.put("counter/complete", message.clone()).res().unwrap();

        },
        _ => println!("UNKNOWN NODE TOPIC"),
    }

}


pub fn boot_callback(sample:Sample, node: &mut Node, polygon_list: &mut OrderedMapPolygon, cluster: &mut OrderedMapPairs){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();

            //get random point to give to new node
            let mut rng = rand::thread_rng();
            let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple

            //DO THIS TO AVOID SAME SITES
            // while cluster.sites.values().contains(&point) {
            //     point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // if tuple is in exclude list, generate a new one
            // }

            println!("------------------------------------");
            println!("Giving point {:?}.... to {:?}",point,data.sender_id);
            println!("------------------------------------");

            //find closest node to new point
            let land_owner=closest_point(&cluster,point);
            println!("{}",land_owner);


            //add node to cluster
            cluster.insert(data.sender_id.to_string(),point);

            polygon_list.insert( data.sender_id.to_string(),vec![]);


            let json_message = json!(NewNodeResponse{
                site:point,
                land_owner:land_owner,
                sender_id:node.zid.clone()
            });

            let _ = node.session.put(format!("node/{}/new_reply",data.sender_id), json_message).res();
        }
        _=> println!("UNKNOWN BOOT TOPIC"),

    }
}
pub fn counter_callback(sample:Sample, expected_counter:&mut i32, counter: &mut i32, polygon_list: &mut OrderedMapPolygon, cluster: &mut OrderedMapPairs){
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
            polygon_list.insert(data.sender_id,data.polygon);
                //polygon_list[index]=data.polygon;
        },
        _=> println!("UNKNOWN COUNTER TOPIC"),

    }
}

fn closest_point(pairs:&OrderedMapPairs, site:(f64, f64)) -> String {
    let mut closest_zid = "";
    let mut min_distance = f64::INFINITY;

    for (zid, map_point) in pairs.iter() {
        let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
        if distance < min_distance {
            min_distance = distance;
            closest_zid = zid;
        }
    }

    closest_zid.to_string()
}


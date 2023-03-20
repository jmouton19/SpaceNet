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
            //idk bout this
            *expected_counter=-1;
            *counter=0;
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

            //tell new NODE how many nodes to wait for (my neighbours)
            let message = json!(ExpectedNodes{
            sender_id:node.zid.clone(),
            number:node.neighbours.sites.len()+2,});
            *expected_counter=node.neighbours.sites.len() as i32;
            //this isn't needed...
            //node.session.put(format!("counter/expected_wait"), message.clone()).res().unwrap();


            //request neighbours from neighbours and send it to new node
            //get new message type?
            let message = json!(NeighboursNeighboursRequest{
            new_zid:data.sender_id,
            sender_id:node.zid.clone()});
            for neighbour_id in node.neighbours.sites.keys(){
                node.session.put(format!("node/{}/neighbours_neighbours",neighbour_id), message.clone()).res().unwrap();
            };
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

        if expected_counter==counter {

            //tell boot how many to wait for
            let message = json!(ExpectedNodes{
                number:node.neighbours.sites.len(),
                sender_id:node.zid.clone()});
                node.session.put("counter/expected_wait", message.clone()).res().unwrap();


                //tell all neighbours to calc new voronoi with my new site.
                let message = json!(NewVoronoiRequest{
                new_zid:data.sender_id,
                new_site:node.site,
                sender_id:node.zid.clone()});
                for neighbour_id in node.neighbours.sites.keys() {
                    node.session.put(format!("node/{}/new_voronoi", neighbour_id), message.clone()).res().unwrap();
                };


                //calc my own voronoi with all neighbours.
                let diagram = Voronoi::new(node.site, &node.neighbours);
                draw_voronoi(&diagram.diagram, format!("new_{}", node.session.zid()).as_str());
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
            node.neighbours.sites.insert(data.new_zid.to_string(),data.new_site);
            let diagram = Voronoi::new(node.site,&node.neighbours);
            draw_voronoi(&diagram.diagram,format!("new_{}",node.session.zid()).as_str());
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


pub fn boot_callback(sample:Sample, node: &mut Node, polygon_list: &mut Vec<Vec<(f64, f64)>>, cluster: &mut SiteIdList){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    println!("Topic... {:?}",topic);
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();

            //get random point to give to new node
            let mut rng = rand::thread_rng();
            let mut point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // generate random (f64, f64) tuple
            while cluster.contains(point) {
                point = (rng.gen_range(10.0..=90.0), rng.gen_range(10.0..=90.0)); // if tuple is in exclude list, generate a new one
            }

            println!("------------------------------------");
            println!("Giving point {:?}.... to {:?}",point,data.sender_id);
            println!("------------------------------------");

            //find closest node to new point
            let land_owner=cluster.closest_point(point);

            //add node to cluster
            cluster.sites.insert(data.sender_id.to_string(),point);

            //NB REWORK POLGYON LIST
            //linked hash map?
            polygon_list.push(vec!());


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
pub fn counter_callback(sample:Sample, expected_counter:&mut i32, counter: &mut i32, polygon_list: &mut Vec<Vec<(f64, f64)>>, cluster: &mut SiteIdList){
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

                //this can be wrong ,ORDER IS IMPORTANT! redo with hashmaps
                //polygon_list[index]=data.polygon;
        },
        _=> println!("UNKNOWN COUNTER TOPIC"),

    }
}


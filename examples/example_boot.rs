use std::io;
use std::thread::sleep;
use std::time::Duration;

use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::utils::*;
use SpaceNet::handlers::*;


//
// fn approx_equal_tuples(t1: &[(f64, f64)], t2: &[(f64, f64)]) -> bool {
//     t1.iter()
//         .zip(t2)
//         .all(|((x1, y1), (x2, y2))| (x1 - x2).abs() < 0.0001 && (y1 - y2).abs() < 0.0001)
// }
//
//
// fn approx_equal_lists(l1: &[(f64, f64)], l2: &[(f64, f64)]) -> bool {
//     if l1.len() != l2.len() {
//         return false;
//     }
//
//     l1.iter().zip(l2.iter()).all(|(t1, t2)| approx_equal_tuples(&[*t1], &[*t2]))
// }


fn main() {

    let mut boot_node=Node::new(Config::default());
    println!("boot node online..... {:?}",boot_node.session.zid());

    let counter_subscriber = boot_node.session.declare_subscriber("counter/*").reliable().res().unwrap();
    let boot_subscriber = boot_node.session.declare_subscriber("node/boot/*").reliable().res().unwrap();
    let node_subscriber = boot_node.session.declare_subscriber(format!("node/{}/*", boot_node.session.zid())).reliable().res().unwrap();

    //set boot node point
    boot_node.site= (50.,50.);

    //add boot node to cluster
    let mut cluster=SiteIdPairs{
        sites:vec![boot_node.site],
        ids:vec![boot_node.session.zid()],
    };

    let diagram = Voronoi::new(boot_node.site,&boot_node.neighbours);
    let polygon= diagram.diagram.cells()[0].points().iter().map(|x|(x.x, x.y)).collect();
    let mut polygon_list:Vec<Vec<(f64,f64)>>=vec!();

    let mut correct_polygon_list:Vec<Vec<(f64,f64)>>=vec!();

    polygon_list.push(polygon);
    draw_voronoi_full(&cluster.sites,&polygon_list,"initial");
    let mut draw_count=1;


    loop {
        // Handle messages in the queue
        if let Ok(sample) = boot_subscriber.try_recv(){
            let mut expected_counter=-1;
            let mut counter=0;
            boot_callback(sample, &mut boot_node,&mut polygon_list,&mut cluster);
            // Process the message here

            while expected_counter!=counter {
                while let Ok(sample) = counter_subscriber.try_recv(){
                    counter_callback(sample, &mut expected_counter,&mut counter,&mut polygon_list,&mut cluster);
                    // Process the message here
                }

                while let Ok(sample) = node_subscriber.try_recv(){
                    node_callback(sample, &mut boot_node);
                    // Process the message here
                }
            }
            //redraw total voronoi
            draw_voronoi_full(&cluster.sites,&polygon_list,format!("voronoi{}",draw_count).as_str());


            //draw correct voronoi
            let temp_cluster= SiteIdPairs{
                ids:cluster.ids[1..].to_vec(),
                sites:cluster.sites[1..].to_vec(),
            };
            let diagram = Voronoi::new(cluster.sites[0],&temp_cluster);

            for cell in diagram.diagram.cells() {
                let polygon= cell.points().iter().map(|x|(x.x, x.y)).collect();
                correct_polygon_list.push(polygon);
            }

            draw_voronoi_full(&cluster.sites,&correct_polygon_list,format!("confirm{}",draw_count).as_str());

            // for (i,cell) in correct_polygon_list.iter().enumerate(){
            //     let correct=approx_equal_lists(&correct_polygon_list[i],&polygon_list[i]);
            //     if !correct{
            //         println!("***********************");
            //         println!("THE LISTS ARE NOT EQUAL");
            //         println!("***********************");
            //         println!("{:?}",correct_polygon_list[i]);
            //         println!("{:?}",polygon_list[i]);
            //         break;
            //     }
            // }



            draw_count+=1;
        }

        // while let Ok(sample) = wait_subscriber.try_recv(){
        //     boot_callback(sample, &mut boot_node,&mut cluster,&mut reply_counter);
        //     // Process the message here
        // }
        //
        // while let Ok(sample) = completed_subscriber.try_recv(){
        //     boot_callback(sample, &mut boot_node,&mut cluster,&mut reply_counter);
        //     // Process the message here
        // }




        // let mut input = String::new();
        // io::stdin().read_line(&mut input).expect("Failed to read line");
        // if input.trim() == "q" {
        //     println!("Q is pressed!");
        //     let diagram = Voronoi::new(cluster);
        //     draw_voronoi(&diagram.diagram,"initial",true);
        //     break;
        // }

        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}





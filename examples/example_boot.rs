use std::io;
use std::thread::sleep;
use std::time::Duration;

use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::utils::*;
use SpaceNet::handlers::*;


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

    //calc and draw initial voronoi
    let diagram = Voronoi::new(boot_node.site,&boot_node.neighbours);
    draw_voronoi(&diagram.diagram,"initial",true);

    loop {
        // Handle messages in the queue
        if let Ok(sample) = boot_subscriber.try_recv(){
            let mut expected_counter=-1;
            let mut counter=0;
            boot_callback(sample, &mut boot_node,&mut cluster);
            // Process the message here

            while expected_counter!=counter {
                while let Ok(sample) = counter_subscriber.try_recv(){
                    counter_callback(sample, &mut expected_counter,&mut counter);
                    // Process the message here
                }

                while let Ok(sample) = node_subscriber.try_recv(){
                    node_callback(sample, &mut boot_node);
                    // Process the message here
                }
            }
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





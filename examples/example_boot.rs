use std::thread::sleep;
use std::time::Duration;
use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::utils::*;
use SpaceNet::handlers::*;


fn main() {

    let mut boot_node=Node::new(Config::default());
    println!("boot node online..... {:?}",boot_node.session.zid());
    let boot_subscriber = boot_node.session.declare_subscriber("node/boot/*").reliable().res().unwrap();
    let node_subscriber = node.session.declare_subscriber(format!("node/{}/*", node.session.zid())).reliable().res().unwrap();

    //draw initial voronoi
    boot_node.site= (50.,50.);
    let mut cluster=SiteIdPairs{
        sites:vec![boot_node.site],
        ids:vec![boot_node.session.zid()],
    };

    let diagram = voronoi(boot_node.site,&boot_node.neighbours.sites);
    draw_voronoi(&diagram);

    loop {
        // Handle messages in the queue
        while let Ok(sample) = boot_subscriber.try_recv(){
            boot_callback(sample, &mut boot_node,&mut cluster);
            // Process the message here
        }
        while let Ok(sample) = node_subscriber.try_recv(){
            node_callback(sample, &mut boot_node);
            // Process the message here
        }
        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        sleep(Duration::from_secs(1));
    }
}





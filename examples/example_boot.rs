use space_net::handlers::*;
use space_net::node::*;
use space_net::utils::*;
use std::collections::HashMap;

fn main() {

    let mut boot_server=BootNode::new_with_node(Node::new(Config::default()));
    //let mut boot_node=&boot_server.node.unwrap();
    //let mut boot_node = Node::new(Config::default());

    println!("boot node online..... {:?}", boot_server.node.zid);

    loop {
        boot_server.run();
        // Handle messages in the queue

        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}

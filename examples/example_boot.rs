use space_net::node::*;

fn main() {
    let mut boot_server = BootNode::new_with_node(Node::new(Config::default()));
    println!("boot node online..... {:?}", boot_server.node.zid);
    loop {
        // Handle messages in the queue
        boot_server.run();
        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}

use space_net::node::*;

fn main() {
    let mut boot_server = BootNode::new_with_node(Node::new(Config::default(), "network_1"));
    println!("boot node online..... {:?}", boot_server.node.get_zid());
    loop {
        boot_server.run();
    }
}

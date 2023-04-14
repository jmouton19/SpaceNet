use space_net::node::*;

fn main() {
    let mut node = Node::new(Config::default(), "network_1").leave_on_pressed('q');
    println!("node online..... {:?}", node.get_zid());
    loop {
        if !node.is_running() {
            break;
        }
        node.run();
    }
}

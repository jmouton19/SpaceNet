use space_net::node::*;

fn main() {
    //create node instance and join the cluster
    let mut node = Node::new(Config::default(), "network_1").leave_on_pressed('q');
    //node.join();

    println!("node online..... {:?}", node.get_zid());

    loop {
        if !node.is_running() {
            break;
        }
        node.run();
        // other tasks
    }
}

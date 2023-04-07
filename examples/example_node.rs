use space_net::node::*;

fn main() {
    //join overlay network
    let mut node = Node::new(Config::default()).leave_on_pressed('q');
    println!("node online..... {:?}", node.zid);

    //leave_on_pressed(node.session.clone(), 'q');

    loop {
        if !node.running {
            break;
        }
        node.run();
        // other tasks
    }
}

use rand::Rng;
use space_net::node::*;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());
    loop {
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

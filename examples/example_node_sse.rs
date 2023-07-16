use rand::Rng;
use space_net::node::*;
use std::thread;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());
    node.add_player("jeff");
    node.add_player("bob");
    node.add_player("cj");
    node.remove_player("cj");

    loop {
        node.update_player("jeff", rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
        thread::sleep(std::time::Duration::from_secs(3));
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

use rand::Rng;
use space_net::node::*;
use space_net::subscriber::NodeSubscriber;
use std::thread;
use std::time::Duration;
use zenoh::subscriber::Subscriber;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());

    let sub = NodeSubscriber::new(&node);
    let subclone = sub.clone();
    async_std::task::spawn_blocking(move || {
        sub.subscribe("pog");
        loop {
            thread::sleep(Duration::from_secs(1));
            let output = sub.receive();
            println!("Output: {:?}", output);
        }
    });

    async_std::task::spawn_blocking(move || {
        subclone.subscribe("pog2");
        loop {
            thread::sleep(Duration::from_secs(1));
            let output = subclone.receive();
            println!("Output: {:?}", output);
        }
    });

    loop {
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

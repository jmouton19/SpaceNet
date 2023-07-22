use rand::Rng;
use space_net::node::*;
use space_net::subscriber::Subscriber;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());

    let sub = Subscriber::new();
    let topic = format!("{}/test", node.get_cluster_name());
    async_std::task::spawn_blocking(move || {
        sub.subscribe(topic.as_str());
        loop {
            thread::sleep(Duration::from_secs(1));
            let output = sub.receive();
            if output.get_payload().len() != 0 {
                println!("Output: {:?}", output);
            }
        }
    });
    loop {
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

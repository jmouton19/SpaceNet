use rand::Rng;
use space_net::node::*;

use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());

    thread::sleep(Duration::from_secs(1));

    let mut i = 1;
    while i <= 10 {
        let payload: Vec<u8> = (0..10).map(|_| rand::thread_rng().gen::<u8>()).collect();
        node.send_message(payload, "pog");
        println!("Payload {} sent!", i);
        i += 1;
        // let payload: Vec<u8> = (0..10).map(|_| rand::thread_rng().gen::<u8>()).collect();
        // node.send_message(payload, "", "pog");
        // println!("Payload {} sent!", i);
        // i += 1;
    }

    loop {
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

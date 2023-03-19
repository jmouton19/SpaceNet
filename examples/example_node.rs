use std::thread::sleep;
use std::time::Duration;


use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::utils::*;
use SpaceNet::handlers::*;


fn main() {
    let mut node = Node::new(Config::default());
    println!("node online..... {:?}", node.session.zid());
    let node_subscription = node.session.declare_subscriber(format!("node/{}/*", node.session.zid())).reliable().res().unwrap();

    let message = json!(NewNodeRequest{
        sender_id:node.session.zid(),
    });

    //message boot node
    node.session.put("node/boot/new", message).res().unwrap();

    loop {
        // Handle messages in the queue
        while let Ok(sample) = node_subscription.try_recv() {
            node_callback(sample, &mut node);
            // Process the message here
        }

        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        sleep(Duration::from_secs(1));
    }
}


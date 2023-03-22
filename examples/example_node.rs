// use std::thread::sleep;
// use std::time::Duration;
use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::handlers::*;


fn main() {
    //join overlay network
    let mut node = Node::new(Config::default());
    println!("node online..... {:?}", node.zid);
    let node_subscription = node.session.declare_subscriber(format!("node/{}/*", node.zid)).reliable().res().unwrap();

    //message boot node
    let message = json!(NewNodeRequest{
        sender_id:node.zid.clone(),
    });
    node.session.put("node/boot/new", message).res().unwrap();

    loop {
        // Handle messages in the queue
        while let Ok(sample) = node_subscription.try_recv() {
            node_callback(sample, &mut node);
            // Process the message here
        }
        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}


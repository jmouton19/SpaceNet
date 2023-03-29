// use std::thread::sleep;
// use std::time::Duration;
use SpaceNet::node::*;
use SpaceNet::message::*;
use SpaceNet::handlers::*;
use async_std::io;
use async_std::io::ReadExt;
use async_std::task;

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


    //if press q msg boot
    let mut closure_session =node.session.clone();
    let closure_id=node.zid.clone();
    task::spawn(async move {
        let mut buffer = [0; 1];
        loop {
            // Read a single byte from stdin
            if let Ok(()) = io::stdin().read_exact(&mut buffer).await {
                if buffer[0] == b'q' {
                    // Call the function when the user presses 'q'
                    let message = json!(NewNodeRequest{
            sender_id:closure_id,
            });
                    closure_session.put("node/boot/leave_request", message).res().unwrap();
                    break;
                }
            }
        }
    });

    loop {

        // Handle messages in the queue
        while let Ok(sample) = node_subscription.try_recv() {
            node_callback(sample, &mut node);
            // Process the message here
        }

        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}


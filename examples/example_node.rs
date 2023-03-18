use std::thread::sleep;
use std::time::Duration;


use SpaceNet::node::*;
use SpaceNet::message::*;


fn main() {

    let mut node = Node::new(Config::default());
    println!("node online..... {:?}",node.session.zid());

    let node_subscription  = node.session.declare_subscriber(format!("node/{}/*",node.session.zid())).reliable().res().unwrap();

    let message = json!(NewNodeRequest{
        value:"Hello im new".to_string(),
        sender_id:node.session.zid().to_string(),
    });

    //message boot node
    node.session.put("node/boot/new",message).res().unwrap();

    loop {
        // Handle messages in the queue
        while let Ok(sample) = node_subscription.try_recv(){
            node_callback(sample, &mut node);
            // Process the message here
        }

        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        sleep(Duration::from_secs(1));
    }
}


fn node_callback(sample:Sample,node:&mut Node){

    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");

    match topic {
        "new" => {
            let data: NewNodeResponse = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("Given point.... {:?}",data.site);
        },

        _ => println!("What topic is that lmao"),
    }
}
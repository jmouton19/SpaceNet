use std::thread::sleep;
use std::time::Duration;
use SpaceNet::node::*;
use SpaceNet::message::*;


fn main() {

    let mut boot_node=Node::new(Config::default());

    let boot_subscriber = boot_node.session.declare_subscriber("node/boot/*").reliable().res().unwrap();

    loop {
        // Handle messages in the queue
        while let Ok(sample) = boot_subscriber.try_recv(){
            boot_callback(sample, &mut boot_node);
            // Process the message here
        }
        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        sleep(Duration::from_secs(1));
    }


}


fn boot_callback(sample:Sample,node: &mut Node){
    let topic=sample.key_expr.split('/').nth(2).unwrap_or("");
    match topic {
        "new" => {
            let data: NewNodeRequest = serde_json::from_str(&sample.value.to_string()).unwrap();
            println!("{}.... from {}",data.value,data.sender_id);

            let json_message = json!(NewNodeResponse{
                value:"New node acknowledged... ".to_string(),
                site:(69.,69.),
            });

            let _ = node.session.put(format!("node/{}/new",data.sender_id), json_message).res();
        }
        _=> println!("what topic is that lmao?"),

    }
}

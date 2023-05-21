use async_std::task::sleep;
use space_net::node::*;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let mut node = Node::new("network_1");
    //node.leave_on_pressed('q');
    node.start_async().await;
    println!("node online..... {:?}", node.get_zid());

    loop {
        // if node.get_status() == NodeStatus::Offline {
        //     break;
        // }
        sleep(Duration::from_secs(1)).await;
        println!("Doing other stuff...");
    }
}

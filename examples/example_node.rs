use space_net::node::*;

fn main() {
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join();
    loop {
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

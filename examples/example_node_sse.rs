use rand::Rng;
use space_net::node::*;
use std::thread;

fn main() {
    let mut rng = rand::thread_rng();
    let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
    let mut node = Node::new("network_1");
    node.leave_on_pressed('q');
    node.join(point);
    println!("Node online..... {:?}", node.get_zid());

    let mut player_list =vec![];
    for i in 0..65 {
        let player_id=uuid::Uuid::new_v4().to_string();
        player_list.push(player_id.clone());
        node.add_player(&player_id);
    }

    loop {

       for player_id in &player_list {
            node.update_player(player_id, rng.gen_range(5.0..=95.0), rng.gen_range(5.0..=95.0));
        };
        thread::sleep(std::time::Duration::from_millis(100));
        if node.get_status() == NodeStatus::Offline {
            break;
        }
    }
}

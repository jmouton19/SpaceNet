use space_net::boot_node::*;

fn main() {
    let mut boot_server = BootNode::new("network_1", true);
    println!("boot node online..... {:?}", boot_server.get_zid());
    loop {
        boot_server.run();
    }
}

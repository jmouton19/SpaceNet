use async_std::io;
use async_std::io::ReadExt;
use async_std::task;
use space_net::handlers::*;
use space_net::message::*;
use space_net::node::*;

fn main() {
    //join overlay network
    let mut node = Node::new(Config::default());
    println!("node online..... {:?}", node.zid);

    //if press q msg boot
    let closure_session = node.session.clone();
    let closure_id = node.zid.clone();
    task::spawn(async move {
        let mut buffer = [0; 1];
        loop {
            // Read a single byte from stdin
            if let Ok(()) = io::stdin().read_exact(&mut buffer).await {
                if buffer[0] == b'q' {
                    // Call the function when the user presses 'q'
                    let message = json!(NewNodeRequest {
                        sender_id: closure_id,
                    });
                    closure_session
                        .put("node/boot/leave_request", message)
                        .res()
                        .unwrap();
                    break;
                }
            }
        }
    });

    loop{
        if !node.running{
            break;
        }
        node.run();
        // other tasks
    }

}

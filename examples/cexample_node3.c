#include "space_net.h"
#include "stdio.h"

int main() {
    printf("CNode example...\n");
    const char* cluster_name = "network_1";
    void* node_ptr = new_node(cluster_name);
    leave_on_key(node_ptr,'q');

    const char* zid_str = get_zid_node(node_ptr);
    printf("Node online... %s\n", zid_str);
    join(node_ptr,69.0,69.0);


//    let sub = NodeSubscriber::new(&node);
//    let subclone=sub.clone();
//    async_std::task::spawn_blocking(move || {
//            sub.subscribe("pog");
//            loop {
//                thread::sleep(Duration::from_secs(1));
//                let output = sub.receive();
//                println!("Output: {:?}", output);
//            }
//    });

    while(1) {
        if (get_status(node_ptr) == Offline) {
            break;
        }
        // run(node_ptr);
    }

    return 0;
}
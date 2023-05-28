#include "space_net.h"
#include "stdio.h"

int main() {
    printf("CNode example...\n");
    const char* cluster_name = "my-cluster";
    void* node_ptr = new_node(cluster_name,69.0,69.0);
    leave_on_key(node_ptr,'q');

    const char* zid_str = get_zid_node(node_ptr);
    printf("Node online... %s\n", zid_str);
    join(node_ptr);

    while(1) {
        if (get_status(node_ptr) == Offline) {
            break;
        }
       // run(node_ptr);
    }

    return 0;
}
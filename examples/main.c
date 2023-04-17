#include "space_net.h"
#include "stdio.h"

int main() {
    const char* cluster_name = "my-cluster";
    void* node_ptr = new(cluster_name);

    // Get the ZID from the node
    const char* zid_str = get_zid(node_ptr);

    // Print the ZID
    printf("ZID: %s\n", zid_str);
    // your Rust library function calls here
    return 0;
}
#include "space_net.h"
#include "stdio.h"

int main() {
    const char* cluster_name = "network_1";
    void* boot_node_ptr = new_boot(cluster_name,false);
    const char* zid_str = get_zid_boot(boot_node_ptr);
    printf("Boot node online... %s\n", zid_str);
    free_c_string(zid_str);

    while(1) {
       // run_boot(boot_node_ptr);
    }

    return 0;
}
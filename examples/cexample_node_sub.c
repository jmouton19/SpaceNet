#include "space_net.h"
#include "stdio.h"
#include <string.h>

int main() {
    printf("CNode example...\n");
    const char* cluster_name = "network_1";
    void* node_ptr = new_node(cluster_name);
    leave_on_key(node_ptr,'q');

    const char* zid_str = get_zid_node(node_ptr);
    printf("Node online... %s\n", zid_str);
    free_c_string(zid_str);
    join(node_ptr,69.0,69.0);

    void* sub=new_subscriber();
    char string[100];
    strcpy(string, cluster_name);
    strcat(string, "/test");
    subscribe(sub,string);

    while(1) {
        void* message=receive(sub);
        Buffer buffer=get_payload(message);
        if(buffer.len>0){
            printf("\nPayload received: %s",buffer.data);
        }
        if (get_status(node_ptr) == Offline) {
            free_node(node_ptr);
            break;
        }
        free_payload_message(message);
    }
    return 0;
}
#include "space_net.h"
#include "stdio.h"


unsigned char *gen_rdm_bytestream (size_t num_bytes)
{
    unsigned char *stream = malloc (num_bytes);
    size_t i;

    for (i = 0; i < num_bytes; i++)
    {
        stream[i] = rand ();
    }

    return stream;
}



int main() {
    printf("CNode example...\n");
    const char* cluster_name = "network_1";
    void* node_ptr = new_node(cluster_name);
    leave_on_key(node_ptr,'q');

    const char* zid_str = get_zid_node(node_ptr);
    printf("Node online... %s\n", zid_str);
    join(node_ptr,69.0,69.0);


    int i=1;
    while (i<=10) {
        int size=10;
        Buffer buffer;
        buffer.data=gen_rdm_bytestream(size);
        buffer.len=size;
        send_message(node_ptr,buffer,"pog");
        printf("\nPayload %d sent!\n",i);

        i+=1;
        buffer.data=gen_rdm_bytestream(size);
        send_message(node_ptr,buffer,"pog2");
        printf("Payload %d sent!\n",i);
        i+=1;
    }


    while(1) {
        if (get_status(node_ptr) == Offline) {
            break;
        }
        // run(node_ptr);
    }

    return 0;
}
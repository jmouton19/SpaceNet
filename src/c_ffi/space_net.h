#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Joining,
  Online,
  Leaving,
  Offline,
} NodeStatus;

typedef struct {
  uint8_t *data;
  uintptr_t len;
} Buffer;

const char *closest_neighbour(void *node_ptr, double site_x, double site_y);

void free_boot_node(void *node);

void free_c_string(char *s);

void free_node(void *node);

void free_subscriber(void *node);

NodeStatus get_status(void *node_ptr);

const char *get_zid_boot(void *boot_ptr);

const char *get_zid_node(void *node_ptr);

int is_in_polygon(void *node_ptr, double x, double y);

int is_neighbour(void *node_ptr, const char *zid);

void join(void *node_ptr, double site_x, double site_y);

void leave(void *node_ptr);

void leave_on_key(void *node_ptr, char key);

void *new_boot(const char *cluster_name, bool centralized_voronoi);

void *new_node(const char *cluster_name);

void *new_subscriber(const void *node_ptr);

Buffer receive(const void *subscriber_ptr);

void send_message(void *node_ptr, Buffer buffer, const char *receiver_node, const char *topic);

void subscribe(const void *subscriber_ptr, const char *topic, int global_sub);

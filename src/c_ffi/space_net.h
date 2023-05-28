#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum NodeStatus {
  Joining,
  Online,
  Leaving,
  Offline,
} NodeStatus;

void *new_node(const char *cluster_name, double site_x, double site_y);

void *new_boot(const char *cluster_name, bool centralized_voronoi);

void leave_on_key(void *node_ptr, char key);

void leave(void *node_ptr);

const char *get_zid_node(void *node_ptr);

enum NodeStatus get_status(void *node_ptr);

const char *get_zid_boot(void *boot_ptr);

int is_neighbour(void *node_ptr, const char *zid);

int is_in_polygon(void *node_ptr, double x, double y);

void join(void *node_ptr);

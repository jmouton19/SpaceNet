use crate::boot_node::BootNodeData;
use crate::message::NewVoronoiResponse;

use bincode::deserialize;
use std::sync::MutexGuard;

/// Handles a task completed message from a node. Increments the amount of received messages. Boot node updates the polygon list and cluster of boot node.
pub fn handle_task_completed(payload: &[u8], mut boot_node_data: MutexGuard<BootNodeData>) {
    boot_node_data.received_counter += 1;
    let data: NewVoronoiResponse = deserialize(payload).unwrap();
    boot_node_data
        .polygon_list
        .insert(data.sender_id.clone(), data.polygon);
    boot_node_data.cluster.insert(data.sender_id, data.site);
}

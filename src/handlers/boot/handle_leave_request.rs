use crate::boot_node::BootNodeData;
use crate::message::DefaultMessage;
use crate::node::SyncResolve;
use bincode::{deserialize, serialize};
use std::sync::{Arc, MutexGuard};
use zenoh::Session;

/// Handles a leave request from a node. Boot node removes the node from the cluster and polygon list.
pub fn handle_leave_request(
    payload: &[u8],
    mut boot_node_data: MutexGuard<BootNodeData>,
    session: Arc<Session>,
) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    println!("Node... {} wants to leave....", data.sender_id);
    session
        .put(
            format!(
                "{}/node/{}/leave_reply",
                boot_node_data.cluster_name, data.sender_id
            ),
            serialize(&true).unwrap(),
        )
        .res()
        .unwrap();
    boot_node_data.polygon_list.remove(data.sender_id.as_str());
    boot_node_data.cluster.remove(data.sender_id.as_str());
}

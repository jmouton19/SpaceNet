use crate::boot_node::BootNode;
use crate::message::DefaultMessage;
use crate::node::SyncResolve;
use bincode::{deserialize, serialize};


/// Handles a leave request from a node. Boot node removes the node from the cluster and polygon list.
pub fn handle_leave_request(payload: &[u8], boot_node: &mut BootNode) {
    let data: DefaultMessage = deserialize(payload).unwrap();
    println!("Node... {} wants to leave....", data.sender_id);
    boot_node
        .session
        .put(
            format!(
                "{}/node/{}/leave_reply",
                boot_node.cluster_name, data.sender_id
            ),
            serialize(&true).unwrap(),
        )
        .res()
        .unwrap();
    boot_node.polygon_list.remove(data.sender_id.as_str());
    boot_node.cluster.remove(data.sender_id.as_str());
}

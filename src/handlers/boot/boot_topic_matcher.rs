use crate::boot_node::BootNodeData;
use crate::handlers::boot::handle_join_request::handle_join_request;
use crate::handlers::boot::handle_leave_request::handle_leave_request;
use crate::handlers::boot::handle_task_completed::handle_task_completed;
use crate::handlers::boot::set_expected_counter::set_expected_counter;
use std::sync::Arc;
use zenoh::Session;

pub fn boot_topic_matcher(
    topic: &str,
    payload: &[u8],
    boot_node_data: &mut BootNodeData,
    session: &Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    match topic {
        "new" => {
            handle_join_request(payload, boot_node_data, session, cluster_name, zid);
        }
        "leave_request" => {
            handle_leave_request(payload, boot_node_data, session, cluster_name);
        }
        _ => println!("UNKNOWN BOOT TOPIC"),
    }
}

pub fn boot_counter_topic_matcher(topic: &str, payload: &[u8], boot_node_data: &mut BootNodeData) {
    match topic {
        "expected_wait" => {
            set_expected_counter(payload, &mut boot_node_data.expected_counter);
        }
        "complete" => {
            handle_task_completed(payload, boot_node_data);
        }
        _ => println!("UNKNOWN COUNTER TOPIC"),
    }
}

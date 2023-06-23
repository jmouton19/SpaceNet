use crate::handlers::node::handle_leave_response::handle_leave_response;
use crate::handlers::node::handle_leave_voronoi_request::handle_leave_voronoi_request;
use crate::handlers::node::handle_neighbours_neighbours_request::handle_neighbours_neighbours_request;
use crate::handlers::node::handle_neighbours_neighbours_response::handle_neighbours_neighbours_response;
use crate::handlers::node::handle_new_voronoi_request::handle_new_voronoi_request;
use crate::handlers::node::handle_owner_request::handle_owner_request;
use crate::handlers::node::handle_owner_response::handle_owner_response;
use crate::node::NodeData;
use bincode::deserialize;
use std::sync::Arc;
use zenoh::Session;

pub fn node_topic_matcher(
    topic: &str,
    payload: &[u8],
    node_data: &mut NodeData,
    session: Arc<Session>,
    zid: &str,
    cluster_name: &str,
) {
    match topic {
        "new_reply" => {
            handle_owner_request(payload, node_data, session, zid, cluster_name);
        }
        "owner_request" => {
            handle_owner_request(payload, node_data, session, zid, cluster_name);
        }
        "owner_response" => {
            handle_owner_response(payload, node_data, session, zid, cluster_name);
        }
        "neighbours_neighbours" => {
            handle_neighbours_neighbours_request(payload, node_data, session, zid, cluster_name);
        }
        "neighbours_neighbours_reply" => {
            handle_neighbours_neighbours_response(payload, node_data, session, zid, cluster_name);
        }
        "new_voronoi" => {
            handle_new_voronoi_request(payload, node_data, session, zid, cluster_name);
        }
        "leave_voronoi" => {
            handle_leave_voronoi_request(payload, node_data, session, zid, cluster_name);
        }
        "leave_reply" => {
            handle_leave_response(payload, node_data, session, zid, cluster_name);
        }
        _ => println!("UNKNOWN NODE TOPIC"),
    }
}

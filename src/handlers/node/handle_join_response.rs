use crate::message::{NewNodeResponse, NewResponse, NewVoronoiRequest};
use crate::node::{Node, SyncResolve};
use bincode::{deserialize, serialize};

/// Sets site given from boot node and messages land owner to request neighbour list
pub fn handle_join_response(payload: &[u8], node: &mut Node) {
    let data: NewResponse = deserialize(payload).unwrap();

    //set site to given site
    node.site = data.new_site;

    node.neighbours = data.neighbours;
}

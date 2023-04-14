use crate::message::DefaultMessage;
use crate::node::{Node, SyncResolve};
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use bincode::{deserialize, serialize};

pub fn handle_leave_request(
    payload: &[u8],
    node: &mut Node,
    polygon_list: &mut OrderedMapPolygon,
    cluster: &mut OrderedMapPairs,
) {
    let data: DefaultMessage = deserialize(payload.as_ref()).unwrap();
    println!("Node... {} wants to leave....", data.sender_id);
    node.session
        .put(
            format!("{}/node/{}/leave_reply", node.cluster, data.sender_id),
            serialize(&true).unwrap(),
        )
        .res()
        .unwrap();
    polygon_list.remove(data.sender_id.as_str());
    cluster.remove(data.sender_id.as_str());
}

use crate::message::NewVoronoiResponse;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use bincode::deserialize;

/// Handles a task completed message from a node. Increments the amount of received messages. Boot node updates the polygon list and cluster of boot node.
pub fn handle_task_completed(
    payload: &[u8],
    counter: &mut i32,
    polygon_list: &mut OrderedMapPolygon,
    cluster: &mut OrderedMapPairs,
) {
    *counter += 1;
    let data: NewVoronoiResponse = deserialize(payload).unwrap();
    polygon_list.insert(data.sender_id.clone(), data.polygon);
    cluster.insert(data.sender_id, data.site);
}

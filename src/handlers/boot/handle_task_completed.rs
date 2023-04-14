use crate::message::NewVoronoiResponse;
use crate::types::OrderedMapPolygon;
use bincode::deserialize;

pub fn handle_task_completed(
    payload: &[u8],
    counter: &mut i32,
    polygon_list: &mut OrderedMapPolygon,
) {
    *counter += 1;
    let data: NewVoronoiResponse = deserialize(payload.as_ref()).unwrap();
    polygon_list.insert(data.sender_id, data.polygon);
}

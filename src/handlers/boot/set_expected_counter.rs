use crate::message::ExpectedNodes;
use bincode::deserialize;

pub fn set_expected_counter(payload:&[u8],expected_counter: &mut i32) {
    let data: ExpectedNodes = deserialize(payload.as_ref()).unwrap();
    println!("Im waiting for {} nodes to reply...", data.number);
    *expected_counter = data.number;
}

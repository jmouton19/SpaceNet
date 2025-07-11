use crate::message::ExpectedNodes;
use bincode::deserialize;

/// Sets the expected counter to the number of nodes that are expected to message boot node before processing the next message.
pub fn set_expected_counter(payload: &[u8], expected_counter: &mut i32) {
    let data: ExpectedNodes = deserialize(payload).unwrap();
    println!("Im waiting for {} nodes to reply...", data.number);
    *expected_counter = data.number;
}

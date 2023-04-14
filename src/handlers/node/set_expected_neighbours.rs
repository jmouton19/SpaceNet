use crate::message::ExpectedNodes;
use crate::node::Node;
use bincode::deserialize;

pub fn set_expected_neighbours(payload: &[u8], node: &mut Node) {
    let data: ExpectedNodes = deserialize(payload.as_ref()).unwrap();
    node.expected_counter = data.number;
    println!("Im expecting {:?} neighbor responses...", data.number);
}

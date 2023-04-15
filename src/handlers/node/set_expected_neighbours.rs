use crate::message::ExpectedNodes;
use crate::node::Node;
use bincode::deserialize;

/// Sets the number of expected neighbours, ie how many messages to wait for before calculating voronoi.
pub fn set_expected_neighbours(payload: &[u8], node: &mut Node) {
    let data: ExpectedNodes = deserialize(payload).unwrap();
    node.expected_counter = data.number;
    println!("Im expecting {:?} neighbor responses...", data.number);
}

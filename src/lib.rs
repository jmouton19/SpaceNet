//! # space_net
//!
//! A native rust library for distributed virtual environments using spatial partitioning.
//!
//! ## Examples
//!
//!
//!### Boot Node
//! Starts a [boot node](node/struct.BootNode.html) (with a node site) listening on cluster `network_1`
//! ```rust
//! use space_net::node::*;
//!
//! fn main() {
//!     let mut boot_server = BootNode::new_with_node(Node::new(Config::default(), "network_1"));
//!    println!("boot node online..... {:?}", boot_server.node.get_zid());
//!     loop {
//!         boot_server.run();
//!     }
//! }
//!```
//!### Node
//! Starts a [node](node/struct.Node.html) listening on cluster `network_1` which leaves the cluster on pressing `q`.
//! ```rust
//! use space_net::node::*;
//!
//! fn main() {
//!     let mut node = Node::new(Config::default(), "network_1").leave_on_pressed('q');
//!     println!("node online..... {:?}", node.get_zid());
//!     loop {
//!         if !node.is_running() {
//!            break;
//!         }
//!         node.run();
//!     }
//! }
//! ```
//!
//!

pub(crate) mod handlers;
pub(crate) mod message;
pub mod node;
pub(crate) mod old_handlers;
pub(crate) mod types;
pub(crate) mod utils;

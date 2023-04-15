//! # space_net
//!
//! A native rust library for distributed virtual environments using spatial partitioning.
//!
//! ## Examples
//!
//!
//!### Boot Node
//! Starts a [boot node](node/struct.BootNode.html) (with a node site) listening on cluster `network_1`
//! ```rust,no_run
//! use space_net::boot_node::*;
//!
//! fn main() {
//!     let mut boot_server = BootNode::new(Config::default(), "network_1");
//!     println!("boot node online..... {:?}", boot_server.get_zid());
//!     loop {
//!         boot_server.run();
//!     }
//! }
//!```
//!### Node
//! Starts a [node](node/struct.Node.html) listening on cluster `network_1` which leaves the cluster on pressing `q`.
//! ```rust,no_run
//! use space_net::node::*;
//!
//! fn main() {
//!     let mut node = Node::new(Config::default(), "network_1").leave_on_pressed('q');
//!     println!("node online..... {:?}", boot_server.get_zid());
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

pub mod boot_node;
pub(crate) mod handlers;
pub(crate) mod message;
pub mod node;
pub mod types;
pub mod utils;

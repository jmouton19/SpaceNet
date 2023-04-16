#![doc(html_logo_url = "../LOGO.png", html_favicon_url = "../LOGO.png")]
//![SpaceNet](./space_net/index.html) is a native rust library for distributed virtual environments using spatial partitioning.
//! Start a single boot node to handle new nodes joining the cluster. Nodes can be started after the boot node is online.
//!## Dependencies
//! ## Ubuntu Linux
//! ```bash
//! sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
//! ```
//! ## Examples
//!### Boot Node
//! Starts a [boot node](boot_node/struct.BootNode.html) listening on cluster `network_1` for nodes to join.
//! ```rust,no_run
//! use space_net::boot_node::*;
//!
//! fn main() {
//!     let mut boot_server = BootNode::new("network_1");
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
//!     let mut node = Node::new("network_1").leave_on_pressed('q');
//!     println!("node online..... {:?}", node.get_zid());
//!     loop {
//!         if node.get_status() == NodeStatus::Offline {
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
pub mod node_ffi;

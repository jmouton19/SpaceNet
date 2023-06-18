use crate::boot_node::BootNode;

use crate::node::{Node, NodeStatus};
use libc::{c_char, c_int};
use std::ffi::{c_void, CStr, CString};

#[repr(C)]
pub enum EventType {
    PlayerMove,
}

#[repr(C)]
pub struct PlayerMoveData {
    pub(crate) start: [f64; 2],
    pub(crate) end: [f64; 2],
}

#[repr(C)]
pub struct ExternalEvent {
    pub(crate) event: EventType,
    pub(crate) data: EventData,
}

#[repr(C)]
pub struct EventData {
    pub(crate) player_move_data: PlayerMoveData,
}

//new node from C
#[no_mangle]
pub extern "C" fn new_node(cluster_name: *const c_char) -> *mut c_void {
    let c_str = unsafe { CStr::from_ptr(cluster_name) };
    let cluster_name = c_str.to_str().unwrap();
    let node = Box::new(Node::new(cluster_name));
    Box::into_raw(node) as *mut c_void
}

//new boot node from C
#[no_mangle]
pub extern "C" fn new_boot(cluster_name: *const c_char, centralized_voronoi: bool) -> *mut c_void {
    let c_str = unsafe { CStr::from_ptr(cluster_name) };
    let cluster_name = c_str.to_str().unwrap();
    let boot_node = Box::new(BootNode::new(cluster_name, centralized_voronoi));
    Box::into_raw(boot_node) as *mut c_void
}
// leave node when key is pressed from C
#[no_mangle]
pub extern "C" fn leave_on_key(node_ptr: *mut c_void, key: c_char) {
    let node = unsafe { &*(node_ptr as *mut Node) };
    let key = key as u8 as char;
    node.leave_on_pressed(key);
}

// leave node from C
#[no_mangle]
pub extern "C" fn leave(node_ptr: *mut c_void) {
    let node = unsafe { &mut *(node_ptr as *mut Node) };
    node.leave();
}

// get zid from C
#[no_mangle]
pub extern "C" fn get_zid_node(node_ptr: *mut c_void) -> *const c_char {
    let node = unsafe { &*(node_ptr as *mut Node) };
    let zid_str = node.get_zid();
    let c_string = CString::new(zid_str).unwrap();
    c_string.into_raw()
}

//get node status from C
#[no_mangle]
pub extern "C" fn get_status(node_ptr: *mut c_void) -> NodeStatus {
    let node = unsafe { &*(node_ptr as *mut Node) };
    node.get_status()
}

// get zid boot from C
#[no_mangle]
pub extern "C" fn get_zid_boot(boot_ptr: *mut c_void) -> *const c_char {
    let boot = unsafe { &*(boot_ptr as *mut BootNode) };
    let zid_str = boot.get_zid();
    let c_string = CString::new(zid_str).unwrap();
    c_string.into_raw()
}

// //get neighbours from C
// #[no_mangle]
// pub extern "C" fn get_neighbours(node_ptr: *mut c_void) -> *const c_char {
//     let node = unsafe { &*(node_ptr as *mut Node) };
// }

// Check if the node is a neighbour from c
#[no_mangle]
pub extern "C" fn is_neighbour(node_ptr: *mut c_void, zid: *const c_char) -> c_int {
    let c_str = unsafe { CStr::from_ptr(zid) };
    let zid = c_str.to_str().unwrap();
    let node = unsafe { &*(node_ptr as *mut Node) };
    if node.is_neighbour(zid) {
        1
    } else {
        0
    }
}

// /// Get the polygon of the node
// pub fn get_polygon(&self) -> Vec<(f64, f64)> {
//     self.polygon.clone()
// }

// Check if the point site is in the polygon from c
#[no_mangle]
pub extern "C" fn is_in_polygon(node_ptr: *mut c_void, x: f64, y: f64) -> c_int {
    let node = unsafe { &*(node_ptr as *mut Node) };
    let point = (x, y);
    if node.is_in_polygon(point) {
        1
    } else {
        0
    }
}

// //run boot node from C
// #[no_mangle]
// pub extern "C" fn run_boot(boot_ptr: *mut c_void) {
//     let boot = unsafe { &mut *(boot_ptr as *mut BootNode) };
//     boot.run();
// }

// run node from C
#[no_mangle]
pub extern "C" fn join(node_ptr: *mut c_void, site_x: f64, site_y: f64) {
    let node = unsafe { &mut *(node_ptr as *mut Node) };
    node.join((site_x, site_y));
}

#[no_mangle]
pub extern "C" fn send_message(
    node_ptr: *mut c_void,
    external_event: *const ExternalEvent,
    receiver_node: *const c_char,
) {
    let node = unsafe { &mut *(node_ptr as *mut Node) };
    let external_event = unsafe { &*external_event };
    let c_str = unsafe { CStr::from_ptr(receiver_node) };
    let receiver = c_str.to_str().unwrap();
    node.send_message(external_event, receiver);
}

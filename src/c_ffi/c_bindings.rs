use crate::boot_node::BootNode;

use crate::node::{Node, NodeStatus};
use crate::subscriber::NodeSubscriber;
use libc::{c_char, c_int};
use std::ffi::{c_void, CStr, CString};

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn new_node(cluster_name: *const c_char) -> *mut c_void {
    let c_str = unsafe { CStr::from_ptr(cluster_name) };
    let cluster_name = c_str.to_str().unwrap();
    let node = Box::new(Node::new(cluster_name));
    Box::into_raw(node) as *mut c_void
}
#[no_mangle]
pub extern "C" fn free_node(node: *mut c_void) {
    unsafe {
        let _ = Box::from_raw(node as *mut Node);
    }
}


//new boot node from C
#[no_mangle]
pub extern "C" fn new_boot(cluster_name: *const c_char, centralized_voronoi: bool) -> *mut c_void {
    let c_str = unsafe { CStr::from_ptr(cluster_name) };
    let cluster_name = c_str.to_str().unwrap();
    let boot_node = Box::new(BootNode::new(cluster_name, centralized_voronoi));
    Box::into_raw(boot_node) as *mut c_void
}
#[no_mangle]
pub extern "C" fn free_boot_node(node: *mut c_void) {
    unsafe {
        let _ = Box::from_raw(node as *mut BootNode);
    }
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
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
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

// run node from C
#[no_mangle]
pub extern "C" fn closest_neighbour(
    node_ptr: *mut c_void,
    site_x: f64,
    site_y: f64,
) -> *const c_char {
    let node = unsafe { &mut *(node_ptr as *mut Node) };
    let zid = node.closest_neighbour((site_x, site_y));
    let c_string = CString::new(zid).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn send_message(
    node_ptr: *mut c_void,
    buffer: Buffer,
    receiver_node: *const c_char,
    topic: *const c_char,
) {
    let node = unsafe { &mut *(node_ptr as *mut Node) };

    let payload_slice = unsafe { std::slice::from_raw_parts(buffer.data, buffer.len) };
    let payload_vec = payload_slice.to_vec();

    let c_str = unsafe { CStr::from_ptr(topic) };
    let topic = c_str.to_str().unwrap();

    let c_str = unsafe { CStr::from_ptr(receiver_node) };
    let receiver = c_str.to_str().unwrap();

    node.send_message(payload_vec, receiver, topic);
}


//subscriber struct
#[no_mangle]
pub extern "C" fn new_subscriber(node_ptr: *const c_void) -> *mut c_void {
    let node = unsafe { &*(node_ptr as *const Node) };
    let sub = Box::new(NodeSubscriber::new(node));
    Box::into_raw(sub) as *mut c_void
}
#[no_mangle]
pub extern "C" fn free_subscriber(node: *mut c_void) {
    unsafe {
        let _ = Box::from_raw(node as *mut NodeSubscriber);
    }
}


//todo!safely convert c_int to i32?

#[no_mangle]
pub extern "C" fn subscribe(subscriber_ptr: *const c_void, topic: *const c_char,global_sub:c_int) {
    let c_str = unsafe { CStr::from_ptr(topic) };
    let topic = c_str.to_str().unwrap();
    let sub = unsafe { &mut*(subscriber_ptr as *mut NodeSubscriber) };
    sub.subscribe(topic,global_sub);
}

#[no_mangle]
pub extern "C" fn receive(subscriber_ptr: *const c_void) -> Buffer {
    let sub = unsafe { &*(subscriber_ptr as *const NodeSubscriber) };
    let mut payload = sub.receive();
    let data_ptr = payload.as_mut_ptr();
    let len = payload.len();
    std::mem::forget(payload);
    Buffer {
        data: data_ptr,
        len,
    }
}

#[no_mangle]
extern "C" fn free_buf(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}

//TODO FREE ALL INTO RAW FUNCTIONS!

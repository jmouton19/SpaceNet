use libc::{c_char, c_double, c_int};
use std::ffi::{c_void, CStr, CString};
use crate::node::Node;

#[no_mangle]
pub extern "C" fn new(cluster_name: *const c_char) -> *mut c_void {
    let c_str = unsafe { CStr::from_ptr(cluster_name) };
    let cluster_name = c_str.to_str().unwrap();
    let node = Box::new(Node::new(cluster_name));
    Box::into_raw(node) as *mut c_void
}

#[no_mangle]
pub extern "C" fn get_zid(node_ptr: *mut c_void) -> *const c_char {
    let node = unsafe { &*(node_ptr as *mut Node) };
    let zid_str = node.get_zid();
    let c_string = CString::new(zid_str).unwrap();
    c_string.into_raw()
}

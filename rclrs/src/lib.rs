use std::{thread, time};

pub mod bindings;
pub mod common;
pub mod context;
pub mod error;
pub mod node;

use crate::bindings::*;
use crate::common::Handle;
use crate::node::Node;

pub fn spin(node: &Node) {
    let context_handle = &mut *node.context.get_mut();
    while unsafe { rcl_context_is_valid(context_handle) } {
        // Sleep here since we don't want a super speed loop here.
        // TODO add `spinOnce`
        let tens = time::Duration::from_millis(1000);
        thread::sleep(tens);
    }
}

use std::cell::{Ref, RefCell, RefMut};
use std::ffi::CString;
use std::ops::Drop;
use std::rc::Rc;

use log::info;

use crate::bindings::*;
use crate::common::Handle;
use crate::context::{Context, ContextHandle};
use crate::error::{RclResult, ToRclResult};

#[derive(Debug)]
pub struct NodeHandle(RefCell<rcl_node_t>);

impl<'a> Handle<rcl_node_t> for &'a NodeHandle {
    type Item = Ref<'a, rcl_node_t>;
    type ItemMut = RefMut<'a, rcl_node_t>;

    fn get(self) -> Self::Item {
        self.0.borrow()
    }
    fn get_mut(self) -> Self::ItemMut {
        self.0.borrow_mut()
    }
}

#[derive(Debug)]
pub struct Node {
    pub handle: Rc<NodeHandle>,
    pub context: Rc<ContextHandle>,
}

impl Node {
    pub fn new(nodename: String, nodens: String, context: &Context) -> Self {
        let node_handle = unsafe { rcl_get_zero_initialized_node() };
        info!("Created Node Handle");
        let node = Node {
            handle: Rc::new(NodeHandle(RefCell::new(node_handle))),
            context: context.handle.clone(),
        };
        node.init(nodename, nodens).unwrap();
        node
    }

    fn init(&self, nodename: String, nodens: String) -> RclResult<()> {
        let node_ops = unsafe { rcl_node_get_default_options() };
        info!("Created node default options");

        let node_name = CString::new(nodename).unwrap();
        let node_ns = CString::new(nodens).unwrap();

        let context_handle = &mut *self.context.get_mut();
        let node_handle = &mut *self.handle.get_mut();
        // create node;
        unsafe {
            rcl_node_init(
                node_handle as *mut _,
                node_name.as_ptr(),
                node_ns.as_ptr(),
                context_handle as *mut _,
                &node_ops as *const _,
            )
            .ok()?;
            info!("Node inited");
        }

        Ok(())
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let node = &mut *self.handle.get_mut();
        unsafe {
            rcl_node_fini(node as *mut _);
            info!("Dropped node");
        }
    }
}

// Use `Option` for node namespace since we want to make it optional with
// default value "".
// Default function arguments in Rust:
// https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust
pub fn create_node(nodename: String, nodens_option: Option<String>) -> Node {
    let context = Context::new();

    let nodens = if let Some(ns) = nodens_option {
        ns
    } else {
        "".to_string()
    };

    let node = Node::new(nodename, nodens, &context);

    node
}

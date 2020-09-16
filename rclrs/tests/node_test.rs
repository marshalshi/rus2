use rclrs::bindings::*;
use rclrs::context::Context;
use rclrs::node::{create_node, Node};

#[test]
fn node_struct() {
    let context = Context::new();
    let _ = Node::new("a".to_string(), "".to_string(), &context);
}

#[test]
fn create_node_some_ns() {
    let nodename = "nodename".to_string();
    let nodens = Some("value".to_string());
    create_node(nodename, nodens);
}

#[test]
fn create_node_none_ns() {
    let nodename = "nodename".to_string();
    let nodens = None;
    create_node(nodename, nodens);
}

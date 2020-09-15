use rclrs;

fn main() {
    let nodename = "thisisnode".to_string();
    let nodens = "thisisns".to_string();
    let node = rclrs::node::create_node(nodename, Some(nodens));

    rclrs::spin(&node);
}

use rclrs::node;

fn main() {
    let nodename = "thisisnode".to_string();
    let nodens = "thisisns".to_string();
    node::create_node(nodename, Some(nodens));
}

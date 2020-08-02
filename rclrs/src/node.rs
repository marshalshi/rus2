use crate::bindings::*;
use std::ffi::CString;
use std::process::Command;
use std::ptr::null_mut;
use std::{thread, time};

pub fn create_node() {
    let mut context = unsafe { rcl_get_zero_initialized_context() };
    println!("1: *** context: {:?} ***", context);

    unsafe {
        let allocator = rcutils_get_default_allocator();
        let mut init_options = rcl_get_zero_initialized_init_options();
        rcl_init_options_init(&mut init_options as *mut _, allocator);

        // `rcl_init` requires `argc` and `argv` parameters for its 1st arg and
        // 2nd arg. But currently we don't have any parameters passed.
        // SO, replace both by default `0` and `NULL`.
        // TODO use args
        let argv_null = null_mut();
        rcl_init(
            0,
            argv_null,
            &init_options as *const _,
            &mut context as *mut _,
        );
        rcl_init_options_fini(&mut init_options as *mut _);
    }

    let mut node = unsafe { rcl_get_zero_initialized_node() };
    println!("2: *** node: {:?} ***", node);

    let node_ops = unsafe { rcl_node_get_default_options() };
    println!("3: *** node_ops: {:?} ***", node_ops);

    let node_name = CString::new("nodename").unwrap();
    let node_ns = CString::new("nodens").unwrap();
    // create node;
    unsafe {
        let ret = rcl_node_init(
            &mut node as *mut _,
            node_name.as_ptr(),
            node_ns.as_ptr(),
            &mut context as *mut _,
            &node_ops as *const _,
        );
        println!("4: *** node init: {:?} ***", ret);
    };

    // Every 1 sec, we run `ros2 node list` to confirm our node
    // is running, since currently we don't have `spin` impelmented.
    for i in 1..6 {
        let tens = time::Duration::from_millis(1000);
        thread::sleep(tens);
        let out = Command::new("sh")
            .arg("-c")
            .arg("ros2 node list")
            .output()
            .expect("failed to execute process");
        println!("5-{}: *** node list: {:?} ***", i, out);
    }

    // fini node;
    unsafe {
        let ret = rcl_node_fini(&mut node as *mut _);
        println!("6: *** fini node: {:?} ***", ret);
    };
}

use rclrs::bindings::*;
use rclrs::error::ToRclResult;
use std::convert::TryInto;

#[test]
fn error_ok() {
    let r: rcl_ret_t = RCL_RET_OK.try_into().unwrap();
    assert!(r.ok().is_ok());
}

#[test]
fn error_not_init() {
    let r: rcl_ret_t = RCL_RET_NOT_INIT.try_into().unwrap();
    assert!(r.ok().is_err());
}

// TODO list other error types.

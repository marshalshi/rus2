use crate::bindings::*;
use std::fmt;

pub type RclResult<T> = Result<T, RclError>;

// We won't try to define different Error here.
// TODO make error more specific.
#[derive(Clone, Debug)]
pub struct RclError;

impl fmt::Display for RclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error code returned")
    }
}

pub trait ToRclResult {
    fn ok(&self) -> RclResult<()>;
}

impl ToRclResult for rcl_ret_t {
    fn ok(&self) -> RclResult<()> {
        if *self as u32 == RCL_RET_OK {
            Ok(())
        } else {
            Err(RclError)
        }
    }
}

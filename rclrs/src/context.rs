use crate::bindings::*;
use crate::common::Handle;
use crate::error::{RclResult, ToRclResult};
use std::cell::{Ref, RefCell, RefMut};
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::rc::Rc;

#[derive(Debug)]
pub struct ContextHandle(RefCell<rcl_context_t>);

impl<'a> Handle<rcl_context_t> for &'a ContextHandle {
    type Item = Ref<'a, rcl_context_t>;
    type ItemMut = RefMut<'a, rcl_context_t>;

    fn get(self) -> Self::Item {
        self.0.borrow()
    }

    fn get_mut(self) -> Self::ItemMut {
        self.0.borrow_mut()
    }
}

#[derive(Debug)]
pub struct Context {
    pub handle: Rc<ContextHandle>,
}

impl Context {
    pub fn new() -> Self {
        let rcontext = unsafe { rcl_get_zero_initialized_context() };
        let mut context = Context {
            handle: Rc::new(ContextHandle(RefCell::new(rcontext))),
        };

        context.init().unwrap();
        context
    }

    fn init(&mut self) -> RclResult<()> {
        let args: Vec<CString> = env::args()
            .filter_map(|arg| CString::new(arg).ok())
            .collect();

        let c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();

        let handle = &mut *self.handle.get_mut();

        unsafe {
            let allocator = rcutils_get_default_allocator();
            let mut init_options = rcl_get_zero_initialized_init_options();
            rcl_init_options_init(&mut init_options as *mut _, allocator);

            rcl_init(
                c_args.len() as i32,
                c_args.as_ptr(),
                &init_options as *const _,
                handle as *mut _,
            )
            .ok()?;
            rcl_init_options_fini(&mut init_options as *mut _).ok()?;
        }
        Ok(())
    }
}

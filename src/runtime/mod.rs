mod array;
mod module;
#[macro_use]
mod packed_func;
mod graph;
mod threading;

use std::{ffi::CStr, os::raw::c_char};

pub use self::{array::*, graph::*, module::*, packed_func::*, threading::*};

#[no_mangle]
pub extern "C" fn TVMAPISetLastError(cmsg: *const c_char) {
  unsafe {
    panic!(CStr::from_ptr(cmsg).to_str().unwrap());
  }
}

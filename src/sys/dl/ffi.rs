use std::mem;

pub use libc::{
   c_char, c_int, c_void
};

pub const RTLD_LAZY:            c_int = 1;
pub const RTLD_NOW:             c_int = 2;

#[repr(C)]
#[derive(Copy)]
pub struct DlInfo {
   dli_fname: *const c_char,
   dli_fbase: *mut c_void,
   dli_sname: *const c_char,
   dli_saddr: *mut c_void
}
impl Clone for DlInfo {
   fn clone(&self) -> Self { *self }
}
impl Default for DlInfo {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

extern "C" {
   pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;

   pub fn dlclose(handle: *mut c_void) -> c_int;

   pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;

   pub fn dladdr(addr: *mut c_void, info: *mut DlInfo) -> c_int;

   pub fn dlerror() -> *mut c_char;
}

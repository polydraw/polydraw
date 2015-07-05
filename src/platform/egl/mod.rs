#![allow(non_camel_case_types)]

use libc::{c_long, c_void, uint64_t, int32_t};

use ::platform::x11::ffi;

pub type khronos_ssize_t = c_long;
pub type khronos_uint64_t = uint64_t;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type EGLint = int32_t;
pub type EGLNativeDisplayType = *mut ffi::XDisplay;
pub type EGLNativePixmapType = *const c_void;
pub type EGLNativeWindowType = ffi::xcb_window_t;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeWindowType = EGLNativeWindowType;

include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

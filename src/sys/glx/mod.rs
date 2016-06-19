#![cfg(target_os = "linux")]

pub mod ffi;

use std::mem;

use error::{RuntimeError, ErrorKind};

use super::x11::ffi::Display as X11Display;


pub struct Display {
   pub ptr: *mut X11Display
}

pub struct Version {
   pub major: ffi::c_int,
   pub minor: ffi::c_int,
}

pub fn initialize(display: &Display) -> Result<Version, RuntimeError> {
   let mut major: ffi::c_int = unsafe {
      mem::uninitialized()
   };
   let mut minor: ffi::c_int = unsafe {
      mem::uninitialized()
   };

   let result = unsafe {
      ffi::glXQueryVersion(display.ptr, &mut major, &mut minor)
   };

   if result == 0 {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "glXQueryVersion failed".to_string()
      ));
   }

   Ok(Version {
      major: major,
      minor: minor
   })
}

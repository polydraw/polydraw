#![cfg(target_os = "linux")]

pub mod ffi;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::utils::fn_ptr::FnPtr;

pub struct Library {
   pub handle: *mut ffi::c_void
}

impl Library {
   pub fn new(name: &str) -> Result<Self, RuntimeError> {
      let cname = try!(CString::new(name));

      let handle = unsafe {
         ffi::dlopen(cname.as_ptr(), ffi::RTLD_LAZY)
      };

      if handle.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::DL,
            format!("Loading dynamic library failed {}", name).to_string()
         ));
      }

      Ok(Library {
         handle: handle,
      })
   }

   pub fn get(&self, name: &str) -> FnPtr {
      let cname = CString::new(name).unwrap();

      unsafe {
         ffi::dlsym(self.handle, cname.as_ptr())
      }
   }
}

impl Drop for Library {
   fn drop(&mut self) {
      unsafe {
         ffi::dlclose(self.handle)
      };
   }
}

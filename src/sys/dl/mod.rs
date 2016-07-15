#![cfg(target_os = "linux")]

pub mod ffi;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::utils::fn_ptr::{FnPtrLoader, FnPtr, FnPtrLibrary};

pub struct Library {
   pub handle: *mut ffi::c_void
}

impl FnPtrLibrary for Library {
   fn open(name: &str) -> Result<Self, RuntimeError> {
      let cname = try!(CString::new(name));

      let handle = unsafe {
         ffi::dlopen(cname.as_ptr(), ffi::RTLD_LAZY)
      };

      if handle.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::DL,
            format!("Opening dynamic library failed {}", name).to_string()
         ));
      }

      Ok(Library {
         handle: handle,
      })
   }
}

impl FnPtrLoader for Library {
   fn load(&self, name: &str) -> FnPtr {
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

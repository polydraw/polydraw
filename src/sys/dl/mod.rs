#![cfg(target_os = "linux")]

pub mod ffi;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::utils::fn_ptr::{FnPtrLoader, FnPtr};

pub struct UnixDynLibrary {
   pub handle: *mut ffi::c_void
}

impl UnixDynLibrary {
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

      Ok(UnixDynLibrary {
         handle: handle,
      })
   }
}

impl Drop for UnixDynLibrary {
   fn drop(&mut self) {
      unsafe {
         ffi::dlclose(self.handle)
      };
   }
}

impl FnPtrLoader for UnixDynLibrary {
   fn load(&self, name: &str) -> FnPtr {
      let cname = CString::new(name).unwrap();

      unsafe {
         ffi::dlsym(self.handle, cname.as_ptr())
      }
   }
}

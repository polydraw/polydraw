use error::{RuntimeError, ErrorKind};

use libc::c_void;

pub type FnPtr = *const c_void;

pub const NULL_PTR: FnPtr = 0 as FnPtr;

pub trait FnPtrLoader {
   fn load(&self, &str) -> FnPtr;

   #[inline]
   fn load_any(&self, names: &[&str]) -> FnPtr {
      for name in names {
         let fn_ptr = self.load(name);
         if fn_ptr != NULL_PTR {
            return fn_ptr;
         }
      }

      NULL_PTR
   }
}

pub trait FnPtrLibrary {
   fn open(&str) -> Result<Self, RuntimeError> where Self: Sized;

   #[inline]
   fn open_any(names: &[&str]) -> Result<Self, RuntimeError> where Self: Sized {
      for name in names {
         match Self::open(name) {
            Ok(result) => {
               return Ok(result);
            },
            Err(_) => {}
         }
      }

      Err(RuntimeError::new(
         ErrorKind::DL,
         format!("Opening any dynamic library failed {:?}", names).to_string()
      ))
   }
}

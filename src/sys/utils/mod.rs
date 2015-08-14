pub mod fn_ptr;

use std::ffi::CStr;

use libc::c_char;


macro_rules! field {
   ($that:ident, $field:ident) => {
      unsafe { (*$that.ptr).$field }
   };
}

#[macro_export]
macro_rules! getter {
   ($name:ident, $restype:ty) => {
      pub fn $name(&self) -> $restype {
         field!(self, $name) as $restype
      }
   }
}

pub fn from_cstr(cstr: *const c_char) -> String {
   unsafe {
      String::from_utf8_unchecked(
         CStr::from_ptr(cstr).to_bytes().to_vec()
      )
   }
}

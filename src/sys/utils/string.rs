use std::ffi::CStr;

use libc::c_char;

pub fn from_cstr(cstr: *const c_char) -> String {
   unsafe {
      String::from_utf8_unchecked(
         CStr::from_ptr(cstr).to_bytes().to_vec()
      )
   }
}

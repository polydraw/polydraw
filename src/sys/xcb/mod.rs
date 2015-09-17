#![cfg(target_os = "linux")]

pub mod ffi;
pub mod atom;
pub mod connection;
pub mod screen;
pub mod event;
pub mod window;

pub use self::connection::Connection;
pub use self::screen::Screen;
pub use self::window::Window;
pub use self::atom::Atom;
pub use self::event::{Event, EventType, EventIterator};

use std::ptr;


#[derive(PartialEq)]
pub struct XID {
   pub id: ffi::c_uint
}

fn error_check(
   c: *mut ffi::xcb_connection_t,
   cookie: ffi::xcb_void_cookie_t
) -> Option<ffi::c_uchar> {

   let error = unsafe {
      ffi::xcb_request_check(c, cookie)
   };

   if error != ptr::null_mut() {
      return Some(
         unsafe {
            (*error).error_code
         }
      )
   }

   None
}

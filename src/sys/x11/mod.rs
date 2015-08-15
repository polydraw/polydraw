#![cfg(target_os = "linux")]

pub mod ffi;

use std::ptr;
use std::ffi::{CString, CStr};

use libc::c_char;

use error::{RuntimeError, ErrorKind};

use super::xcb::Connection;

pub struct Display {
   pub ptr: *mut ffi::Display
}

impl Display {
   pub fn default() -> Result<Self, RuntimeError> {
      Display::from_ptr(ptr::null())
   }

   pub fn new<T: Into<Vec<u8>>>(name: T) -> Result<Self, RuntimeError> {
      let c_name = try!(CString::new(name.into()));

      Display::from_ptr(c_name.as_ptr())
   }

   fn from_ptr(name: *const c_char) -> Result<Self, RuntimeError> {
      let display_ptr = unsafe {
         ffi::XOpenDisplay(name)
      };

      if display_ptr.is_null() {
         let description = format!(
            "Opening X display '{}' failed",
            unsafe { CStr::from_ptr(name).to_str().unwrap() }
         );
         return Err(RuntimeError::new(
            ErrorKind::Xlib,
            description
         ));
      }

      Ok(
         Display {
            ptr: display_ptr
         }
      )
   }

   pub fn xcb_connection(&self) -> Result<Connection, RuntimeError> {
      let connection = unsafe {
         ffi::XGetXCBConnection(self.ptr)
      };
      if connection.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::Xlib,
            "Getting XCB connection from display failed".to_string()
         ));
      }

      Ok(
         Connection::new(connection)
      )
   }

   pub fn xcb_own_event_queue(&self) {
      unsafe {
         ffi::XSetEventQueueOwner(
            self.ptr,
            ffi::XCBOwnsEventQueue
         )
      };
   }

   #[inline]
   pub fn default_screen(&self) -> ffi::c_int {
      unsafe {
         (*(self.ptr as ffi::_XPrivDisplay)).default_screen
      }
   }
}

impl Drop for Display {
   fn drop (&mut self) {
      unsafe {
         ffi::XCloseDisplay(self.ptr);
      }
   }
}

#[cfg(test)]
mod test {
   use std::env;
   use super::Display;

   #[test]
   fn test_create_display() {
      assert!(Display::default().is_ok());
   }

   #[test]
   fn test_create_display_from_env() {
      let name = env::var("DISPLAY").unwrap();
      assert!(Display::new(name).is_ok());
   }

   #[test]
   fn test_create_display_error_str() {
      assert!(Display::new("ErrorStringHere").is_err());
   }

   #[test]
   fn test_create_display_nul_str() {
      assert!(Display::new(&b"one\0two"[..]).is_err());
   }

   #[test]
   fn test_create_get_xcb_connection() {
      assert!(Display::default().unwrap().xcb_connection().is_ok());
   }
}


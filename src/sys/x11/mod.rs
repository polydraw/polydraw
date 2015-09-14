#![cfg(target_os = "linux")]

pub mod ffi;

use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::xcb::Connection;

pub struct Display {
   pub ptr: *mut ffi::Display
}

impl Display {
   pub fn default() -> Result<Self, RuntimeError> {
      let display_ptr = unsafe {
         ffi::XOpenDisplay(ptr::null())
      };

      if display_ptr.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::Xlib,
            "Opening default X display failed".to_string()
         ));
      }

      Ok(
         Display {
            ptr: display_ptr
         }
      )
   }

   pub fn new(name: &str) -> Result<Self, RuntimeError> {
      let c_name = match CString::new(name){
         Ok(c_name) => c_name,
         Err(_) => {
            let description = format!(
               "Opening X display with bad display name '{}'", name
            );
            return Err(RuntimeError::new(
               ErrorKind::Xlib,
               description
            ));
         }
      };

      let display_ptr = unsafe {
         ffi::XOpenDisplay(c_name.as_ptr())
      };

      if display_ptr.is_null() {
         let description = format!(
            "Opening X display '{}' failed", name
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
            "Getting XCB connection failed".to_string()
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
   pub fn default_screen(&self) -> ScreenID {
      let screen = unsafe {
         (*(self.ptr as ffi::_XPrivDisplay)).default_screen
      };

      ScreenID {
         screen: screen
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

pub struct ScreenID {
   pub screen: ffi::c_int
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
      assert!(Display::new(name.as_ref()).is_ok());
   }

   #[test]
   fn test_create_display_error_str() {
      assert!(Display::new("ErrorStringHere").is_err());
   }

   #[test]
   fn test_create_display_nul_str() {
      assert!(Display::new("one\0two").is_err());
   }

   #[test]
   fn test_create_get_xcb_connection() {
      assert!(Display::default().unwrap().xcb_connection().is_ok());
   }
}


use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::super::x11::ScreenID;

use super::ffi;
use super::screen::Screen;
use super::XID;
use super::event::{Event, EventIterator};
use super::atom::{InternAtomCookie, InternAtomReply};


pub struct Connection {
   pub ptr: *mut ffi::xcb_connection_t
}

impl Connection {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      Connection {
         ptr: connection_ptr,
      }
   }

   pub fn screen_of_display(&self, screen_id: &ScreenID) -> Result<Screen, RuntimeError> {
      let setup = unsafe { ffi::xcb_get_setup(self.ptr) };

      if setup == ptr::null() {
         return Err(RuntimeError::new(
            ErrorKind::XCB,
            "Getting XCB connection setup failed".to_string()
         ));
      }

      let mut iter = unsafe {
         ffi::xcb_setup_roots_iterator(setup)
      };

      let mut screen_num = screen_id.screen;

      while screen_num > 0 && iter.rem != 0 {
         unsafe { ffi::xcb_screen_next(&mut iter) };
         screen_num -= 1;
      }

      Ok(Screen::new(iter.data))
   }

   pub fn generate_id(&self) -> Result<XID, RuntimeError> {
      let id = unsafe {
         ffi::xcb_generate_id(self.ptr)
      };

      if id == 0xffffffff {
         return Err(RuntimeError::new(
            ErrorKind::XCB,
            "Generating XID failed".to_string()
         ));
      }

      Ok(XID {
         id: id
      })
   }

   pub fn wait_for_event(&self) -> Option<Event> {
      let event_ptr = unsafe {
         ffi::xcb_wait_for_event(self.ptr)
      };

      if event_ptr.is_null() {
         return None;
      }

      Some(
         Event::new(event_ptr)
      )
   }

   pub fn poll_event_iter(&self) -> EventIterator {
      EventIterator::new(self.ptr)
   }

   pub fn flush(&self) {
      unsafe {
         ffi::xcb_flush(self.ptr);
      }
   }

   pub fn intern_atom(&self, name: &str, existing_only: bool) -> InternAtomCookie {
      let c_name = CString::new(name).unwrap();

      let xcb_cookie = unsafe {
         ffi::xcb_intern_atom(
               self.ptr,
               existing_only as ffi::c_uchar,
               name.len() as ffi::c_ushort,
               c_name.as_ptr()
         )
      };

      InternAtomCookie {
         xcb_cookie: xcb_cookie
      }
   }

   pub fn intern_atom_reply(&self, cookie: &InternAtomCookie) -> InternAtomReply {
      let xcb_reply = unsafe {
         ffi::xcb_intern_atom_reply(
            self.ptr,
            cookie.xcb_cookie,
            ptr::null_mut()
         )
      };

      InternAtomReply {
         xcb_reply: xcb_reply
      }
   }
}


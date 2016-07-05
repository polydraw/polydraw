use std::ptr;
use std::rc::Rc;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind, VoidResult};

use super::ffi;
use super::connection::Connection;
use super::XID;
use super::screen::Screen;
use super::atom::Atom;


pub struct Window {
   pub connection: Rc<Connection>,
   pub window_id: XID,
}

impl Window {
   pub fn create(
      connection: &Rc<Connection>,
      screen: &Screen,
      width: u32,
      height: u32,
   ) -> Result<Self, RuntimeError> {
      let window_id = match connection.generate_id() {
         Ok(window_id) => window_id,
         Err(e) => return Err(e)
      };

      let eventmask =
         ffi::XCB_EVENT_MASK_STRUCTURE_NOTIFY |
         ffi::XCB_EVENT_MASK_KEY_PRESS |
         ffi::XCB_EVENT_MASK_BUTTON_PRESS |
         ffi::XCB_EVENT_MASK_BUTTON_RELEASE |
         ffi::XCB_EVENT_MASK_BUTTON_MOTION |
         ffi::XCB_EVENT_MASK_POINTER_MOTION;
      let valuelist = [eventmask, 0];
      let valuemask = ffi::XCB_CW_EVENT_MASK;

      let cookie = unsafe {
         ffi::xcb_create_window_checked(
            connection.ptr,
            ffi::XCB_COPY_FROM_PARENT as u8,
            window_id.id,
            screen.root(),
            0, 0,
            width as ffi::c_ushort, height as ffi::c_ushort,
            0,
            ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            valuemask,
            valuelist.as_ptr()
         )
      };

      match connection.error_check(cookie) {
         Some(error_code) => {
            return Err(RuntimeError::new(
               ErrorKind::XCB,
               format!("Create XCB window failed: {}", error_code)
            ));
         },
         None => {}
      }

      Ok(Window {
         connection: connection.clone(),
         window_id: window_id,
      })
   }

   pub fn map(&self) -> VoidResult {
      let cookie = unsafe {
         ffi::xcb_map_window_checked(self.connection.ptr, self.window_id.id)
      };

      match self.connection.error_check(cookie) {
         Some(error_code) => {
            return Err(RuntimeError::new(
               ErrorKind::XCB,
               format!("Mapping XCB window failed: {}", error_code)
            ));
         },
         None => {}
      }

      Ok(())
   }

   pub fn position(&self, x: i32, y: i32) -> VoidResult {
      let value_mask = ffi::XCB_CONFIG_WINDOW_X | ffi::XCB_CONFIG_WINDOW_Y;
      let value_list = [x as ffi::c_uint, y as ffi::c_uint, 0];

      let cookie = unsafe {
         ffi::xcb_configure_window_checked(
            self.connection.ptr,
            self.window_id.id,
            value_mask as ffi::c_ushort,
            value_list.as_ptr()
         )
      };

      match self.connection.error_check(cookie) {
         Some(error_code) => {
            return Err(RuntimeError::new(
               ErrorKind::XCB,
               format!("Setting XCB window position failed: {}", error_code)
            ));
         },
         None => {}
      }

      Ok(())
   }

   pub fn register_close_event(&self) -> Result<(Atom, Atom), RuntimeError> {
      let protocols_cookie = self.connection.intern_atom("WM_PROTOCOLS", true);
      let protocols_reply = self.connection.intern_atom_reply(&protocols_cookie);

      let delete_window_cookie = self.connection.intern_atom("WM_DELETE_WINDOW", false);
      let delete_window_reply = self.connection.intern_atom_reply(&delete_window_cookie);

      let protocols_atom = protocols_reply.atom();
      let delete_window_atom = delete_window_reply.atom();

      let cookie = unsafe {
         ffi::xcb_change_property_checked(
            self.connection.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            self.window_id.id,
            protocols_atom.xcb_atom,
            ffi::XCB_ATOM_ATOM,
            32,
            1,
            &delete_window_atom.xcb_atom as *const u32 as *const _
         )
      };

      match self.connection.error_check(cookie) {
         Some(error_code) => {
            return Err(RuntimeError::new(
               ErrorKind::XCB,
               format!("Registering close event failed: {}", error_code)
            ));
         },
         None => {}
      }

      Ok((protocols_atom, delete_window_atom))
   }

   pub fn set_title(&self, title: &str) -> VoidResult {
      let c_title = CString::new(title).unwrap();

      let cookie = unsafe {
         ffi::xcb_change_property_checked(
            self.connection.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            self.window_id.id,
            ffi::XCB_ATOM_WM_NAME,
            ffi::XCB_ATOM_STRING,
            8,
            title.len() as ffi::c_uint,
            c_title.as_ptr() as *const _
         )
      };

      match self.connection.error_check(cookie) {
         Some(error_code) => {
            return Err(RuntimeError::new(
               ErrorKind::XCB,
               format!("Setting window title failed: {}", error_code)
            ));
         },
         None => {}
      }

      Ok(())
   }

   pub fn query_pointer(&self) -> QueryPointerCookie {
      let xcb_cookie = unsafe {
         ffi::xcb_query_pointer(
            self.connection.ptr,
            self.window_id.id,
         )
      };

      QueryPointerCookie {
         xcb_cookie: xcb_cookie
      }
   }

   pub fn query_pointer_reply(&self, cookie: &QueryPointerCookie) -> QueryPointerReply {
      let xcb_reply = unsafe {
         ffi::xcb_query_pointer_reply(
            self.connection.ptr,
            cookie.xcb_cookie,
            ptr::null_mut()
         )
      };

      QueryPointerReply {
         xcb_reply: xcb_reply
      }
   }
}

impl Drop for Window {
   fn drop (&mut self) {
      unsafe {
         ffi::xcb_destroy_window(
            self.connection.ptr, self.window_id.id
         );
      }
   }
}

pub struct QueryPointerCookie {
   pub xcb_cookie: ffi::xcb_query_pointer_cookie_t
}

pub struct QueryPointerReply {
   pub xcb_reply: *mut ffi::xcb_query_pointer_reply_t
}

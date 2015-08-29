#![cfg(target_os = "linux")]

pub mod ffi;

use std::ptr;
use std::rc::Rc;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::x11::ScreenID;

pub struct Connection {
   pub ptr: *mut ffi::xcb_connection_t
}

impl Connection {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      Connection {
         ptr: connection_ptr,
      }
   }

   pub fn screen_of_display(&self, screen_id: &ScreenID) -> Screen {
      let mut iter = unsafe {
         ffi::xcb_setup_roots_iterator(
            ffi::xcb_get_setup(self.ptr)
         )
      };

      let mut screen_num = screen_id.screen;

      while screen_num > 0 && iter.rem != 0 {
         unsafe { ffi::xcb_screen_next(&mut iter) };
         screen_num -= 1;
      }

      Screen::new(iter.data)
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

#[derive(PartialEq)]
pub struct XID {
   pub id: ffi::c_uint
}

pub struct Screen {
   pub ptr: *mut ffi::xcb_screen_t
}

impl Screen {
   pub fn new(screen_ptr: *mut ffi::xcb_screen_t) -> Self {
      Screen {
         ptr: screen_ptr,
      }
   }

   getter!(root, ffi::xcb_window_t);

   getter!(default_colormap, ffi::xcb_colormap_t);

   getter!(white_pixel, ffi::c_uint);

   getter!(black_pixel, ffi::c_uint);

   getter!(current_input_masks, ffi::c_uint);

   getter!(width_in_pixels, ffi::c_ushort);

   getter!(height_in_pixels, ffi::c_ushort);

   getter!(width_in_millimeters, ffi::c_ushort);

   getter!(height_in_millimeters, ffi::c_ushort);

   getter!(min_installed_maps, ffi::c_ushort);

   getter!(max_installed_maps, ffi::c_ushort);

   getter!(root_visual, ffi::xcb_visualid_t);

   getter!(backing_stores, ffi::c_uchar);

   getter!(save_unders, ffi::c_uchar);

   getter!(root_depth, ffi::c_uchar);

   getter!(allowed_depths_len, ffi::c_uchar);
}

pub enum EventType {
   KeyPress,
   KeyRelease,
   ButtonPress,
   ButtonRelease,
   MotionNotify,
   EnterNotify,
   LeaveNotify,
   FocusIn,
   FocusOut,
   KeymapNotify,
   Expose,
   ClientMessage,
   ConfigureNotify,
   Empty,
   Unidentified,
}

impl EventType {
   pub fn new(xcb_type: ffi::c_uchar) -> Self {
      match xcb_type {
         ffi::XCB_KEY_PRESS => EventType::KeyPress,
         ffi::XCB_KEY_RELEASE => EventType::KeyRelease,
         ffi::XCB_BUTTON_PRESS => EventType::ButtonPress,
         ffi::XCB_BUTTON_RELEASE => EventType::ButtonRelease,
         ffi::XCB_MOTION_NOTIFY => EventType::MotionNotify,
         ffi::XCB_ENTER_NOTIFY => EventType::EnterNotify,
         ffi::XCB_LEAVE_NOTIFY => EventType::LeaveNotify,
         ffi::XCB_FOCUS_IN => EventType::FocusIn,
         ffi::XCB_FOCUS_OUT => EventType::FocusOut,
         ffi::XCB_KEYMAP_NOTIFY => EventType::KeymapNotify,
         ffi::XCB_EXPOSE => EventType::Expose,
         ffi::XCB_CLIENT_MESSAGE => EventType::ClientMessage,
         ffi::XCB_CONFIGURE_NOTIFY => EventType::ConfigureNotify,
         _ => EventType::Unidentified
      }
   }

   pub fn empty() -> Self {
      EventType::Empty
   }
}

pub struct Event {
   pub ptr: *mut ffi::xcb_generic_event_t
}

impl Event {
   pub fn new(event_ptr: *mut ffi::xcb_generic_event_t) -> Self {
      Event {
         ptr: event_ptr,
      }
   }

   pub fn empty() -> Self {
      Event {
         ptr: ptr::null_mut(),
      }
   }

   pub fn event_type(&self) -> EventType {
      if self.ptr.is_null() {
         return EventType::empty();
      }

      EventType::new(
         unsafe {
            (*self.ptr).response_type & !0x80
         }
      )
   }

   pub fn is_close_event(
      &self,
      protocols_atom: &Atom,
      delete_window_atom: &Atom
   ) -> bool {
      unsafe {
         let ptr = self.ptr as *mut ffi::xcb_client_message_event_t;

         let data = (*ptr).data.data32();

         if (*ptr).format != 32 ||
            (*ptr)._type != protocols_atom.xcb_atom ||
            (*data)[0] != delete_window_atom.xcb_atom {
            return false;
         }

         true
      }
   }

   pub fn resize_properties(&self) -> (XID, u32, u32) {
      unsafe {
         let ptr = self.ptr as *mut ffi::xcb_configure_notify_event_t;

         let window_id = XID {
            id: (*ptr).window
         };

         (window_id, (*ptr).width as u32, (*ptr).height as u32)
      }
   }
}

impl Drop for Event {
   fn drop (&mut self) {
      unsafe {
         ffi::free(self.ptr as *mut _);
      }
   }
}

pub struct EventIterator {
   ptr: *mut ffi::xcb_connection_t,
   started: bool,
   completed: bool,
}

impl EventIterator {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      EventIterator {
         ptr: connection_ptr,
         started: false,
         completed: false,
      }
   }
}

impl Iterator for EventIterator {
   type Item = Result<Event, RuntimeError>;

   fn next(&mut self) -> Option<Result<Event, RuntimeError>> {
      if self.completed {
         return None;
      }

      let event_ptr = unsafe {
         if !self.started {
            self.started = true;
            ffi::xcb_poll_for_event(self.ptr)
         } else {
            ffi::xcb_poll_for_queued_event(self.ptr)
         }
      };

      if event_ptr.is_null() {
         self.completed = true;

         if unsafe { ffi::xcb_connection_has_error(self.ptr) } != 0 {
            return Some(
               Err(RuntimeError::new(
                  ErrorKind::XCB,
                  "Poll event failed".to_string()
               ))
            );
         }

         return Some(
            Ok(Event::empty())
         );
      }

      Some(
         Ok(Event::new(event_ptr))
      )
   }
}

pub struct Window {
   pub connection: Rc<Connection>,
   pub window_id: XID,
}

impl Window {
   pub fn create(
      connection: &Rc<Connection>,
      screen: &Screen,
      x: u32,
      y: u32,
      width: u32,
      height: u32,
   ) -> Result<Self, RuntimeError> {
      let window_id = match connection.generate_id() {
         Ok(window_id) => window_id,
         Err(e) => return Err(e)
      };

      let eventmask = ffi::XCB_EVENT_MASK_EXPOSURE |
         ffi::XCB_EVENT_MASK_KEY_PRESS |
         ffi::XCB_EVENT_MASK_STRUCTURE_NOTIFY;
      let valuelist = [eventmask, 0];
      let valuemask = ffi::XCB_CW_EVENT_MASK;

      unsafe {
         ffi::xcb_create_window(
            connection.ptr,
            ffi::XCB_COPY_FROM_PARENT as u8,
            window_id.id,
            screen.root(),
            x as ffi::c_short, y as ffi::c_short,
            width as ffi::c_ushort, height as ffi::c_ushort,
            0,
            ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            valuemask,
            valuelist.as_ptr()
         )
      };

      Ok(Window {
         connection: connection.clone(),
         window_id: window_id,
      })
   }

   pub fn map(&self) {
      unsafe {
         ffi::xcb_map_window(self.connection.ptr, self.window_id.id)
      };
   }

   pub fn register_close_event(&self) -> (Atom, Atom) {
      let protocols_cookie = self.connection.intern_atom("WM_PROTOCOLS", true);
      let protocols_reply = self.connection.intern_atom_reply(&protocols_cookie);

      let delete_window_cookie = self.connection.intern_atom("WM_DELETE_WINDOW", false);
      let delete_window_reply = self.connection.intern_atom_reply(&delete_window_cookie);

      let protocols_atom = protocols_reply.atom();
      let delete_window_atom = delete_window_reply.atom();

      unsafe {
         ffi::xcb_change_property(
            self.connection.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            self.window_id.id,
            protocols_atom.xcb_atom,
            ffi::XCB_ATOM_ATOM,
            32,
            1,
            &delete_window_atom.xcb_atom as *const u32 as *const _
         );
      }

      (protocols_atom, delete_window_atom)
   }

   pub fn set_title(&self, title: &str) {
      let c_title = CString::new(title).unwrap();

      unsafe {
         ffi::xcb_change_property(
            self.connection.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            self.window_id.id,
            ffi::XCB_ATOM_WM_NAME,
            ffi::XCB_ATOM_STRING,
            8,
            title.len() as ffi::c_uint,
            c_title.as_ptr() as *const _
         );
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

pub struct Atom {
   xcb_atom: ffi::xcb_atom_t
}

pub struct InternAtomCookie {
   xcb_cookie: ffi::xcb_intern_atom_cookie_t
}

pub struct InternAtomReply {
   xcb_reply: *mut ffi::xcb_intern_atom_reply_t
}

impl InternAtomReply {
   pub fn atom(&self) -> Atom {
      let xcb_atom = unsafe { (*(self.xcb_reply)).atom };
      Atom {
         xcb_atom: xcb_atom
      }
   }
}

use std::fmt;

use error::{RuntimeError, ErrorKind};

use super::ffi;
use super::atom::Atom;
use super::XID;


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
}

impl EventType {
   pub fn new(xcb_type: ffi::c_uchar) -> Option<Self> {
      Some(match xcb_type {
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
         _ => return None
      })
   }
}

impl fmt::Display for EventType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let type_str = match *self {
         EventType::KeyPress => "KeyPress",
         EventType::KeyRelease => "KeyRelease",
         EventType::ButtonPress => "ButtonPress",
         EventType::ButtonRelease => "ButtonRelease",
         EventType::MotionNotify => "MotionNotify",
         EventType::EnterNotify => "EnterNotify",
         EventType::LeaveNotify => "LeaveNotify",
         EventType::FocusIn => "FocusIn",
         EventType::FocusOut => "FocusOut",
         EventType::KeymapNotify => "KeymapNotify",
         EventType::Expose => "Expose",
         EventType::ClientMessage => "ClientMessage",
         EventType::ConfigureNotify => "ConfigureNotify",
      };

      write!(f, "{}", type_str)
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

   pub fn event_type(&self) -> Option<EventType> {
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

   pub fn mouse_move_properties(&self) -> (i32, i32) {
      unsafe {
         let ptr = self.ptr as *mut ffi::xcb_motion_notify_event_t;

         ((*ptr).event_x as i32, (*ptr).event_y as i32)
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
}

impl EventIterator {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      EventIterator {
         ptr: connection_ptr,
         started: false,
      }
   }
}

impl Iterator for EventIterator {
   type Item = Result<Event, RuntimeError>;

   fn next(&mut self) -> Option<Result<Event, RuntimeError>> {
      let event_ptr = unsafe {
         if !self.started {
            self.started = true;
            ffi::xcb_poll_for_event(self.ptr)
         } else {
            ffi::xcb_poll_for_queued_event(self.ptr)
         }
      };

      if event_ptr.is_null() {
         if unsafe { ffi::xcb_connection_has_error(self.ptr) } != 0 {
            return Some(
               Err(RuntimeError::new(
                  ErrorKind::XCB,
                  "Poll event failed".to_string()
               ))
            );
         }

         return None;
      }

      Some(
         Ok(Event::new(event_ptr))
      )
   }
}

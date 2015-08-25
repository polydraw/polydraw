#![cfg(target_os = "linux")]

pub mod ffi;

use std::ptr;

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

   pub fn generate_id(&self) -> ffi::c_uint {
      unsafe {
         ffi::xcb_generate_id(self.ptr)
      }
   }

   pub fn create_window(
      &self,
      wid: ffi::xcb_window_t,
      screen: &Screen,
      x: ffi::c_short, y: ffi::c_short,
      width: ffi::c_ushort, height: ffi::c_ushort,
   ) {
      let eventmask = ffi::XCB_EVENT_MASK_EXPOSURE |
         ffi::XCB_EVENT_MASK_KEY_PRESS |
         ffi::XCB_EVENT_MASK_STRUCTURE_NOTIFY;
      let valuelist = [eventmask, 0];
      let valuemask = ffi::XCB_CW_EVENT_MASK;

      unsafe {
         ffi::xcb_create_window(
            self.ptr,
            ffi::XCB_COPY_FROM_PARENT as u8,
            wid,
            screen.root(),
            x, y,
            width, height,
            0,
            ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            valuemask,
            valuelist.as_ptr()
         )
      };
   }

   pub fn map_window(&self, window: ffi::xcb_window_t) {
      unsafe {
         ffi::xcb_map_window(self.ptr, window)
      };
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

   pub fn destroy_window(&self, window: ffi::xcb_window_t) {
      unsafe {
         ffi::xcb_destroy_window(self.ptr, window);
      }
   }

   pub fn flush(&self) {
      unsafe {
         ffi::xcb_flush(self.ptr);
      }
   }

   pub fn register_close_event(&self, wid: ffi::xcb_window_t) -> (ffi::xcb_atom_t, ffi::xcb_atom_t) {
      unsafe {
         let protocols_cookie = ffi::xcb_intern_atom(
            self.ptr,
            true as ffi::c_uchar,
            12,
            b"WM_PROTOCOLS\0" as *const u8 as *const _
         );

         let protocols_reply = ffi::xcb_intern_atom_reply(
            self.ptr,
            protocols_cookie,
            ptr::null_mut()
         );

         let delete_window_cookie = ffi::xcb_intern_atom(
            self.ptr,
            false as ffi::c_uchar,
            16,
            b"WM_DELETE_WINDOW\0" as *const u8 as *const _
         );

         let delete_window_reply = ffi::xcb_intern_atom_reply(
            self.ptr,
            delete_window_cookie,
            ptr::null_mut()
         );

         let protocols_atom = (*protocols_reply).atom;
         let delete_window_atom = (*delete_window_reply).atom;

         ffi::xcb_change_property(
            self.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            wid,
            protocols_atom,
            ffi::XCB_ATOM_ATOM,
            32,
            1,
            &delete_window_atom as *const u32 as *const _
         );

         (protocols_atom, delete_window_atom)
      }
   }
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
      protocols_atom: ffi::xcb_atom_t,
      delete_window_atom: ffi::xcb_atom_t
   ) -> bool {
      unsafe {
         let ptr = self.ptr as *mut ffi::xcb_client_message_event_t;

         let data = (*ptr).data.data32();

         if (*ptr).format != 32 ||
            (*ptr)._type != protocols_atom ||
            (*data)[0] != delete_window_atom {
            return false;
         }

         true
      }
   }

   pub fn resize_properties(&self) -> (ffi::xcb_window_t, usize, usize) {
      unsafe {
         let ptr = self.ptr as *mut ffi::xcb_configure_notify_event_t;

         ((*ptr).window, (*ptr).width as usize, (*ptr).height as usize)
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

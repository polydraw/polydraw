#![cfg(target_os = "linux")]

pub mod ffi {
   #![allow(non_camel_case_types)]

   use std::mem;

   pub use libc::{
      c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_void, free
   };

   pub enum xcb_connection_t { }

   pub type xcb_keycode_t = c_uchar;
   pub type xcb_window_t = c_uint;
   pub type xcb_colormap_t = c_uint;
   pub type xcb_visualid_t = c_uint;
   pub type xcb_atom_t = c_uint;

   pub const XCB_EVENT_MASK_NO_EVENT:               c_uint = 0;
   pub const XCB_EVENT_MASK_KEY_PRESS:              c_uint = 1;
   pub const XCB_EVENT_MASK_KEY_RELEASE:            c_uint = 2;
   pub const XCB_EVENT_MASK_BUTTON_PRESS:           c_uint = 4;
   pub const XCB_EVENT_MASK_BUTTON_RELEASE:         c_uint = 8;
   pub const XCB_EVENT_MASK_ENTER_WINDOW:           c_uint = 16;
   pub const XCB_EVENT_MASK_LEAVE_WINDOW:           c_uint = 32;
   pub const XCB_EVENT_MASK_POINTER_MOTION:         c_uint = 64;
   pub const XCB_EVENT_MASK_POINTER_MOTION_HINT:    c_uint = 128;
   pub const XCB_EVENT_MASK_BUTTON_1_MOTION:        c_uint = 256;
   pub const XCB_EVENT_MASK_BUTTON_2_MOTION:        c_uint = 512;
   pub const XCB_EVENT_MASK_BUTTON_3_MOTION:        c_uint = 1024;
   pub const XCB_EVENT_MASK_BUTTON_4_MOTION:        c_uint = 2048;
   pub const XCB_EVENT_MASK_BUTTON_5_MOTION:        c_uint = 4096;
   pub const XCB_EVENT_MASK_BUTTON_MOTION:          c_uint = 8192;
   pub const XCB_EVENT_MASK_KEYMAP_STATE:           c_uint = 16384;
   pub const XCB_EVENT_MASK_EXPOSURE:               c_uint = 32768;
   pub const XCB_EVENT_MASK_VISIBILITY_CHANGE:      c_uint = 65536;
   pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY:       c_uint = 131072;
   pub const XCB_EVENT_MASK_RESIZE_REDIRECT:        c_uint = 262144;
   pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY:    c_uint = 524288;
   pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT:  c_uint = 1048576;
   pub const XCB_EVENT_MASK_FOCUS_CHANGE:           c_uint = 2097152;
   pub const XCB_EVENT_MASK_PROPERTY_CHANGE:        c_uint = 4194304;
   pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE:       c_uint = 8388608;
   pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON:      c_uint = 16777216;

   pub const XCB_CW_BACK_PIXMAP:                    c_uint = 1;
   pub const XCB_CW_BACK_PIXEL:                     c_uint = 2;
   pub const XCB_CW_BORDER_PIXMAP:                  c_uint = 4;
   pub const XCB_CW_BORDER_PIXEL:                   c_uint = 8;
   pub const XCB_CW_BIT_GRAVITY:                    c_uint = 16;
   pub const XCB_CW_WIN_GRAVITY:                    c_uint = 32;
   pub const XCB_CW_BACKING_STORE:                  c_uint = 64;
   pub const XCB_CW_BACKING_PLANES:                 c_uint = 128;
   pub const XCB_CW_BACKING_PIXEL:                  c_uint = 256;
   pub const XCB_CW_OVERRIDE_REDIRECT:              c_uint = 512;
   pub const XCB_CW_SAVE_UNDER:                     c_uint = 1024;
   pub const XCB_CW_EVENT_MASK:                     c_uint = 2048;
   pub const XCB_CW_DONT_PROPAGATE:                 c_uint = 4096;
   pub const XCB_CW_COLORMAP:                       c_uint = 8192;
   pub const XCB_CW_CURSOR:                         c_uint = 16384;

   pub const XCB_COPY_FROM_PARENT:                  c_uint = 0;

   pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT:     c_uint = 0;
   pub const XCB_WINDOW_CLASS_INPUT_OUTPUT:         c_uint = 1;
   pub const XCB_WINDOW_CLASS_INPUT_ONLY:           c_uint = 2;

   pub const XCB_PROP_MODE_REPLACE:                c_uchar = 0;
   pub const XCB_PROP_MODE_PREPEND:                c_uchar = 1;
   pub const XCB_PROP_MODE_APPEND:                 c_uchar = 2;

   pub const XCB_KEY_PRESS:                        c_uchar = 2;
   pub const XCB_KEY_RELEASE:                      c_uchar = 3;
   pub const XCB_BUTTON_PRESS:                     c_uchar = 4;
   pub const XCB_BUTTON_RELEASE:                   c_uchar = 5;
   pub const XCB_MOTION_NOTIFY:                    c_uchar = 6;
   pub const XCB_ENTER_NOTIFY:                     c_uchar = 7;
   pub const XCB_LEAVE_NOTIFY:                     c_uchar = 8;
   pub const XCB_FOCUS_IN:                         c_uchar = 9;
   pub const XCB_FOCUS_OUT:                        c_uchar = 10;
   pub const XCB_KEYMAP_NOTIFY:                    c_uchar = 11;
   pub const XCB_EXPOSE:                           c_uchar = 12;
   pub const XCB_GRAPHICS_EXPOSURE:                c_uchar = 13;
   pub const XCB_NO_EXPOSURE:                      c_uchar = 14;
   pub const XCB_VISIBILITY_NOTIFY:                c_uchar = 15;
   pub const XCB_CREATE_NOTIFY:                    c_uchar = 16;
   pub const XCB_DESTROY_NOTIFY:                   c_uchar = 17;
   pub const XCB_UNMAP_NOTIFY:                     c_uchar = 18;
   pub const XCB_MAP_NOTIFY:                       c_uchar = 19;
   pub const XCB_MAP_REQUEST:                      c_uchar = 20;
   pub const XCB_REPARENT_NOTIFY:                  c_uchar = 21;
   pub const XCB_CONFIGURE_NOTIFY:                 c_uchar = 22;
   pub const XCB_CONFIGURE_REQUEST:                c_uchar = 23;
   pub const XCB_GRAVITY_NOTIFY:                   c_uchar = 24;
   pub const XCB_RESIZE_REQUEST:                   c_uchar = 25;
   pub const XCB_CIRCULATE_NOTIFY:                 c_uchar = 26;
   pub const XCB_PROPERTY_NOTIFY:                  c_uchar = 28;
   pub const XCB_SELECTION_CLEAR:                  c_uchar = 29;
   pub const XCB_SELECTION_REQUEST:                c_uchar = 30;
   pub const XCB_SELECTION_NOTIFY:                 c_uchar = 31;
   pub const XCB_COLORMAP_NOTIFY:                  c_uchar = 32;
   pub const XCB_CLIENT_MESSAGE:                   c_uchar = 33;
   pub const XCB_MAPPING_NOTIFY:                   c_uchar = 34;
   pub const XCB_GE_GENERIC:                       c_uchar = 35;

   pub const XCB_ATOM_NONE:                         c_uint = 0;
   pub const XCB_ATOM_ANY:                          c_uint = 0;
   pub const XCB_ATOM_PRIMARY:                      c_uint = 1;
   pub const XCB_ATOM_SECONDARY:                    c_uint = 2;
   pub const XCB_ATOM_ARC:                          c_uint = 3;
   pub const XCB_ATOM_ATOM:                         c_uint = 4;
   pub const XCB_ATOM_BITMAP:                       c_uint = 5;
   pub const XCB_ATOM_CARDINAL:                     c_uint = 6;
   pub const XCB_ATOM_COLORMAP:                     c_uint = 7;
   pub const XCB_ATOM_CURSOR:                       c_uint = 8;
   pub const XCB_ATOM_CUT_BUFFER0:                  c_uint = 9;
   pub const XCB_ATOM_CUT_BUFFER1:                  c_uint = 10;
   pub const XCB_ATOM_CUT_BUFFER2:                  c_uint = 11;
   pub const XCB_ATOM_CUT_BUFFER3:                  c_uint = 12;
   pub const XCB_ATOM_CUT_BUFFER4:                  c_uint = 13;
   pub const XCB_ATOM_CUT_BUFFER5:                  c_uint = 14;
   pub const XCB_ATOM_CUT_BUFFER6:                  c_uint = 15;
   pub const XCB_ATOM_CUT_BUFFER7:                  c_uint = 16;
   pub const XCB_ATOM_DRAWABLE:                     c_uint = 17;
   pub const XCB_ATOM_FONT:                         c_uint = 18;
   pub const XCB_ATOM_INTEGER:                      c_uint = 19;
   pub const XCB_ATOM_PIXMAP:                       c_uint = 20;
   pub const XCB_ATOM_POINT:                        c_uint = 21;
   pub const XCB_ATOM_RECTANGLE:                    c_uint = 22;
   pub const XCB_ATOM_RESOURCE_MANAGER:             c_uint = 23;
   pub const XCB_ATOM_RGB_COLOR_MAP:                c_uint = 24;
   pub const XCB_ATOM_RGB_BEST_MAP:                 c_uint = 25;
   pub const XCB_ATOM_RGB_BLUE_MAP:                 c_uint = 26;
   pub const XCB_ATOM_RGB_DEFAULT_MAP:              c_uint = 27;
   pub const XCB_ATOM_RGB_GRAY_MAP:                 c_uint = 28;
   pub const XCB_ATOM_RGB_GREEN_MAP:                c_uint = 29;
   pub const XCB_ATOM_RGB_RED_MAP:                  c_uint = 30;
   pub const XCB_ATOM_STRING:                       c_uint = 31;
   pub const XCB_ATOM_VISUALID:                     c_uint = 32;
   pub const XCB_ATOM_WINDOW:                       c_uint = 33;
   pub const XCB_ATOM_WM_COMMAND:                   c_uint = 34;
   pub const XCB_ATOM_WM_HINTS:                     c_uint = 35;
   pub const XCB_ATOM_WM_CLIENT_MACHINE:            c_uint = 36;
   pub const XCB_ATOM_WM_ICON_NAME:                 c_uint = 37;
   pub const XCB_ATOM_WM_ICON_SIZE:                 c_uint = 38;
   pub const XCB_ATOM_WM_NAME:                      c_uint = 39;
   pub const XCB_ATOM_WM_NORMAL_HINTS:              c_uint = 40;
   pub const XCB_ATOM_WM_SIZE_HINTS:                c_uint = 41;
   pub const XCB_ATOM_WM_ZOOM_HINTS:                c_uint = 42;
   pub const XCB_ATOM_MIN_SPACE:                    c_uint = 43;
   pub const XCB_ATOM_NORM_SPACE:                   c_uint = 44;
   pub const XCB_ATOM_MAX_SPACE:                    c_uint = 45;
   pub const XCB_ATOM_END_SPACE:                    c_uint = 46;
   pub const XCB_ATOM_SUPERSCRIPT_X:                c_uint = 47;
   pub const XCB_ATOM_SUPERSCRIPT_Y:                c_uint = 48;
   pub const XCB_ATOM_SUBSCRIPT_X:                  c_uint = 49;
   pub const XCB_ATOM_SUBSCRIPT_Y:                  c_uint = 50;
   pub const XCB_ATOM_UNDERLINE_POSITION:           c_uint = 51;
   pub const XCB_ATOM_UNDERLINE_THICKNESS:          c_uint = 52;
   pub const XCB_ATOM_STRIKEOUT_ASCENT:             c_uint = 53;
   pub const XCB_ATOM_STRIKEOUT_DESCENT:            c_uint = 54;
   pub const XCB_ATOM_ITALIC_ANGLE:                 c_uint = 55;
   pub const XCB_ATOM_X_HEIGHT:                     c_uint = 56;
   pub const XCB_ATOM_QUAD_WIDTH:                   c_uint = 57;
   pub const XCB_ATOM_WEIGHT:                       c_uint = 58;
   pub const XCB_ATOM_POINT_SIZE:                   c_uint = 59;
   pub const XCB_ATOM_RESOLUTION:                   c_uint = 60;
   pub const XCB_ATOM_COPYRIGHT:                    c_uint = 61;
   pub const XCB_ATOM_NOTICE:                       c_uint = 62;
   pub const XCB_ATOM_FONT_NAME:                    c_uint = 63;
   pub const XCB_ATOM_FAMILY_NAME:                  c_uint = 64;
   pub const XCB_ATOM_FULL_NAME:                    c_uint = 65;
   pub const XCB_ATOM_CAP_HEIGHT:                   c_uint = 66;
   pub const XCB_ATOM_WM_CLASS:                     c_uint = 67;
   pub const XCB_ATOM_WM_TRANSIENT_FOR:             c_uint = 68;

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_screen_t {
      pub root: xcb_window_t,
      pub default_colormap: xcb_colormap_t,
      pub white_pixel: c_uint,
      pub black_pixel: c_uint,
      pub current_input_masks: c_uint,
      pub width_in_pixels: c_ushort,
      pub height_in_pixels: c_ushort,
      pub width_in_millimeters: c_ushort,
      pub height_in_millimeters: c_ushort,
      pub min_installed_maps: c_ushort,
      pub max_installed_maps: c_ushort,
      pub root_visual: xcb_visualid_t,
      pub backing_stores: c_uchar,
      pub save_unders: c_uchar,
      pub root_depth: c_uchar,
      pub allowed_depths_len: c_uchar,
   }
   impl Clone for xcb_screen_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_screen_t {
      fn default() -> Self { unsafe { ::std::mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_screen_iterator_t {
      pub data: *mut xcb_screen_t,
      pub rem: c_int,
      pub index: c_int,
   }
   impl Clone for xcb_screen_iterator_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_screen_iterator_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_setup_t {
      pub status: c_uchar,
      pub pad0: c_uchar,
      pub protocol_major_version: c_ushort,
      pub protocol_minor_version: c_ushort,
      pub length: c_ushort,
      pub release_number: c_uint,
      pub resource_id_base: c_uint,
      pub resource_id_mask: c_uint,
      pub motion_buffer_size: c_uint,
      pub vendor_len: c_ushort,
      pub maximum_request_length: c_ushort,
      pub roots_len: c_uchar,
      pub pixmap_formats_len: c_uchar,
      pub image_byte_order: c_uchar,
      pub bitmap_format_bit_order: c_uchar,
      pub bitmap_format_scanline_unit: c_uchar,
      pub bitmap_format_scanline_pad: c_uchar,
      pub min_keycode: xcb_keycode_t,
      pub max_keycode: xcb_keycode_t,
      pub pad1: [c_uchar; 4usize],
   }
   impl Clone for xcb_setup_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_setup_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_void_cookie_t {
      pub sequence: c_uint,
   }
   impl Clone for xcb_void_cookie_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_void_cookie_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_intern_atom_reply_t {
      pub response_type: c_uchar,
      pub pad0: c_uchar,
      pub sequence: c_ushort,
      pub length: c_uint,
      pub atom: xcb_atom_t,
   }
   impl Clone for xcb_intern_atom_reply_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_intern_atom_reply_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_intern_atom_cookie_t {
      pub sequence: c_uint,
   }
   impl Clone for xcb_intern_atom_cookie_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_intern_atom_cookie_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_generic_event_t {
      pub response_type: c_uchar,
      pub pad0: c_uchar,
      pub sequence: c_ushort,
      pub pad: [c_uint; 7usize],
      pub full_sequence: c_uint,
   }
   impl Clone for xcb_generic_event_t {
      fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_generic_event_t {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_generic_error_t {
       pub response_type: c_uchar,
       pub error_code: c_uchar,
       pub sequence: c_ushort,
       pub resource_id: c_uint,
       pub minor_code: c_ushort,
       pub major_code: c_uchar,
       pub pad0: c_uchar,
       pub pad: [c_uint; 5usize],
       pub full_sequence: c_uint,
   }
   impl Clone for xcb_generic_error_t {
       fn clone(&self) -> Self { *self }
   }
   impl Default for xcb_generic_error_t {
       fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[link(name="xcb")]
   extern "C" {
      pub fn xcb_get_setup(
         c: *mut xcb_connection_t
      ) -> *const xcb_setup_t;

      pub fn xcb_screen_next(
         i: *mut xcb_screen_iterator_t
      ) -> ();

      pub fn xcb_setup_roots_iterator(
         R: *const xcb_setup_t
      ) -> xcb_screen_iterator_t;

      pub fn xcb_generate_id(
         c: *mut xcb_connection_t
      ) -> c_uint;

      pub fn xcb_create_window(
         c: *mut xcb_connection_t,
         depth: c_uchar,
         wid: xcb_window_t,
         parent: xcb_window_t,
         x: c_short,
         y: c_short,
         width: c_ushort,
         height: c_ushort,
         border_width: c_ushort,
         _class: c_ushort,
         visual: xcb_visualid_t,
         value_mask: c_uint,
         value_list: *const c_uint
      ) -> xcb_void_cookie_t;

      pub fn xcb_destroy_window(
         c: *mut xcb_connection_t,
         window: xcb_window_t
      ) -> xcb_void_cookie_t;

      pub fn xcb_map_window(
         c: *mut xcb_connection_t,
         window: xcb_window_t
      ) -> xcb_void_cookie_t;

      pub fn xcb_wait_for_event(
         c: *mut xcb_connection_t
      ) -> *mut xcb_generic_event_t;

      pub fn xcb_poll_for_event(
         c: *mut xcb_connection_t
      ) -> *mut xcb_generic_event_t;

      pub fn xcb_flush(
         c: *mut xcb_connection_t
      ) -> c_int;

      pub fn xcb_change_property(
         c: *mut xcb_connection_t,
         mode: c_uchar,
         window: xcb_window_t,
         property: xcb_atom_t,
         _type: xcb_atom_t,
         format: c_uchar,
         data_len: c_uint,
         data: *const c_void
      ) -> xcb_void_cookie_t;

      pub fn xcb_intern_atom(
         c: *mut xcb_connection_t,
         only_if_exists: c_uchar,
         name_len: c_ushort,
         name: *const c_char
      ) -> xcb_intern_atom_cookie_t;

      pub fn xcb_intern_atom_reply(
         c: *mut xcb_connection_t,
         cookie: xcb_intern_atom_cookie_t,
         e: *mut *mut xcb_generic_error_t
      ) -> *mut xcb_intern_atom_reply_t;

      pub fn xcb_connection_has_error(
         c: *mut xcb_connection_t
      ) -> c_int;
   }
}

use std::ptr;

pub struct Connection {
   pub ptr: *mut ffi::xcb_connection_t
}

impl Connection {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      Connection {
         ptr: connection_ptr,
      }
   }

   pub fn screen_of_display(&self, screen: ffi::c_int) -> Screen {
      let mut iter = unsafe {
         ffi::xcb_setup_roots_iterator(
            ffi::xcb_get_setup(self.ptr)
         )
      };

      let mut screen_num = screen;

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
         ffi::XCB_EVENT_MASK_KEY_PRESS;
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

   pub fn poll_for_event(&self) -> Option<Event> {
      let event_ptr = unsafe {
         ffi::xcb_poll_for_event(self.ptr)
      };

      if event_ptr.is_null() {
         if unsafe { ffi::xcb_connection_has_error(self.ptr) } != 0 {
            return None;
         }

         return Some(
            Event::empty()
         );
      }

      Some(
         Event::new(event_ptr)
      )
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

   pub fn register_close_event(&self, wid: ffi::xcb_window_t) {
      unsafe {
         let cookie_pr = ffi::xcb_intern_atom(
            self.ptr,
            true as ffi::c_uchar,
            12,
            b"WM_PROTOCOLS\0" as *const u8 as *const _
         );

         let reply_pr = ffi::xcb_intern_atom_reply(
            self.ptr,
            cookie_pr,
            ptr::null_mut()
         );

         let cookie_dw = ffi::xcb_intern_atom(
            self.ptr,
            false as ffi::c_uchar,
            16,
            b"WM_DELETE_WINDOW\0" as *const u8 as *const _
         );

         let reply_dw = ffi::xcb_intern_atom_reply(
            self.ptr,
            cookie_dw,
            ptr::null_mut()
         );

         let atom_pr = (*reply_pr).atom;
         let atom_dw = (*reply_dw).atom;

         ffi::xcb_change_property(
            self.ptr,
            ffi::XCB_PROP_MODE_REPLACE,
            wid,
            atom_pr,
            ffi::XCB_ATOM_ATOM,
            32,
            1,
            &atom_dw as *const u32 as *const _
         );
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
}

impl Drop for Event {
   fn drop (&mut self) {
      unsafe {
         ffi::free(self.ptr as *mut _);
      }
   }
}

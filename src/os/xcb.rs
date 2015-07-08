pub mod ffi {
   #![allow(non_camel_case_types)]

   use std::mem;

   pub use libc::{c_uchar, c_short, c_ushort, c_int, c_uint};

   pub enum xcb_connection_t { }

   pub type xcb_keycode_t = c_uchar;
   pub type xcb_window_t = c_uint;
   pub type xcb_colormap_t = c_uint;
   pub type xcb_visualid_t = c_uint;

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

   pub const XCB_CW_BACK_PIXMAP:        c_uint = 1;
   pub const XCB_CW_BACK_PIXEL:         c_uint = 2;
   pub const XCB_CW_BORDER_PIXMAP:      c_uint = 4;
   pub const XCB_CW_BORDER_PIXEL:       c_uint = 8;
   pub const XCB_CW_BIT_GRAVITY:        c_uint = 16;
   pub const XCB_CW_WIN_GRAVITY:        c_uint = 32;
   pub const XCB_CW_BACKING_STORE:      c_uint = 64;
   pub const XCB_CW_BACKING_PLANES:     c_uint = 128;
   pub const XCB_CW_BACKING_PIXEL:      c_uint = 256;
   pub const XCB_CW_OVERRIDE_REDIRECT:  c_uint = 512;
   pub const XCB_CW_SAVE_UNDER:         c_uint = 1024;
   pub const XCB_CW_EVENT_MASK:         c_uint = 2048;
   pub const XCB_CW_DONT_PROPAGATE:     c_uint = 4096;
   pub const XCB_CW_COLORMAP:           c_uint = 8192;
   pub const XCB_CW_CURSOR:             c_uint = 16384;

   pub const XCB_COPY_FROM_PARENT:  c_uint = 0;

   pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT:  c_uint = 0;
   pub const XCB_WINDOW_CLASS_INPUT_OUTPUT:      c_uint = 1;
   pub const XCB_WINDOW_CLASS_INPUT_ONLY:        c_uint = 2;

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
   impl ::std::clone::Clone for xcb_screen_t {
      fn clone(&self) -> Self { *self }
   }
   impl ::std::default::Default for xcb_screen_t {
      fn default() -> Self { unsafe { ::std::mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_screen_iterator_t {
      pub data: *mut xcb_screen_t,
      pub rem: c_int,
      pub index: c_int,
   }
   impl ::std::clone::Clone for xcb_screen_iterator_t {
      fn clone(&self) -> Self { *self }
   }
   impl ::std::default::Default for xcb_screen_iterator_t {
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
   impl ::std::clone::Clone for xcb_setup_t {
      fn clone(&self) -> Self { *self }
   }
   impl ::std::default::Default for xcb_setup_t {
      fn default() -> Self { unsafe { ::std::mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct xcb_void_cookie_t {
      pub sequence: c_uint,
   }
   impl ::std::clone::Clone for xcb_void_cookie_t {
      fn clone(&self) -> Self { *self }
   }
   impl ::std::default::Default for xcb_void_cookie_t {
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
   impl ::std::clone::Clone for xcb_generic_event_t {
      fn clone(&self) -> Self { *self }
   }
   impl ::std::default::Default for xcb_generic_event_t {
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
   }
}

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

   pub fn map_window(
      &self,
      window: ffi::xcb_window_t
   ) {
      unsafe {
         ffi::xcb_map_window(self.ptr, window)
      };
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

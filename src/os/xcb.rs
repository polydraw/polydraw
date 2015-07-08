pub mod ffi {
   #![allow(non_camel_case_types)]

   use std::mem;

   pub use libc::{c_uchar, c_short, c_ushort, c_int, c_uint};

   pub enum xcb_connection_t { }

   pub type xcb_keycode_t = c_uchar;
   pub type xcb_window_t = c_uint;
   pub type xcb_colormap_t = c_uint;
   pub type xcb_visualid_t = c_uint;

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

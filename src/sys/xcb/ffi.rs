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

pub const XCB_CONFIG_WINDOW_X:                   c_uint = 1;
pub const XCB_CONFIG_WINDOW_Y:                   c_uint = 2;
pub const XCB_CONFIG_WINDOW_WIDTH:               c_uint = 4;
pub const XCB_CONFIG_WINDOW_HEIGHT:              c_uint = 8;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH:        c_uint = 16;
pub const XCB_CONFIG_WINDOW_SIBLING:             c_uint = 32;
pub const XCB_CONFIG_WINDOW_STACK_MODE:          c_uint = 64;

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
pub struct xcb_client_message_event_t {
   pub response_type: c_uchar,
   pub format: c_uchar,
   pub sequence: c_ushort,
   pub window: xcb_window_t,
   pub _type: xcb_atom_t,
   pub data: xcb_client_message_data_t,
}
impl Clone for xcb_client_message_event_t {
   fn clone(&self) -> Self { *self }
}
impl Default for xcb_client_message_event_t {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_configure_notify_event_t {
   pub response_type: c_uchar,
   pub pad0: c_uchar,
   pub sequence: c_uchar,
   pub event: xcb_window_t,
   pub window: xcb_window_t,
   pub above_sibling: xcb_window_t,
   pub x: c_short,
   pub y: c_short,
   pub width: c_ushort,
   pub height: c_ushort,
   pub border_width: c_ushort,
   pub override_redirect: c_uchar,
   pub pad1: c_uchar,
}
impl Clone for xcb_configure_notify_event_t {
   fn clone(&self) -> Self { *self }
}
impl Default for xcb_configure_notify_event_t {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct xcb_client_message_data_t {
   pub _bindgen_data_: [u32; 5usize],
}
impl xcb_client_message_data_t {
   pub unsafe fn data8(&mut self) -> *mut [c_uchar; 20usize] {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn data16(&mut self) -> *mut [c_ushort; 10usize] {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn data32(&mut self) -> *mut [c_uint; 5usize] {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
}
impl Clone for xcb_client_message_data_t {
   fn clone(&self) -> Self { *self }
}
impl Default for xcb_client_message_data_t {
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

   pub fn xcb_create_window_checked(
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

   pub fn xcb_map_window_checked(
      c: *mut xcb_connection_t,
      window: xcb_window_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_configure_window_checked(
      c: *mut xcb_connection_t,
      window: xcb_window_t,
      value_mask: c_ushort,
      value_list: *const c_uint
   ) -> xcb_void_cookie_t;

   pub fn xcb_wait_for_event(
      c: *mut xcb_connection_t
   ) -> *mut xcb_generic_event_t;

   pub fn xcb_poll_for_event(
      c: *mut xcb_connection_t
   ) -> *mut xcb_generic_event_t;

   pub fn xcb_poll_for_queued_event(
      c: *mut xcb_connection_t
   ) -> *mut xcb_generic_event_t;

   pub fn xcb_flush(
      c: *mut xcb_connection_t
   ) -> c_int;

   pub fn xcb_change_property_checked(
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

   pub fn xcb_request_check(
      c: *mut xcb_connection_t,
      cookie: xcb_void_cookie_t
   ) -> *mut xcb_generic_error_t;
}

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use libc::{c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long,
   c_ulong, c_void};
use std::mem;

pub enum XDisplay { }
pub enum XPrivate { }
pub enum XrmHashBucketRec { }
pub enum XGC { }

pub enum xcb_connection_t { }

pub type uint8_t = c_uchar;
pub type int16_t = c_short;
pub type uint16_t = c_ushort;
pub type uint32_t = c_uint;

pub type XID = c_ulong;
pub type GC = *mut XGC;
pub type Display = XDisplay;
pub type Colormap = XID;
pub type Window = XID;
pub type XPointer = *mut c_char;
pub type VisualID = c_ulong;

pub type xcb_keycode_t = uint8_t;
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_visualid_t = uint32_t;

pub type XEventQueueOwner = c_uint;

pub const XlibOwnsEventQueue: c_uint = 0;
pub const XCBOwnsEventQueue: c_uint = 1;

pub const XCB_COLORMAP_ALLOC_NONE: c_uint = 0;
pub const XCB_COLORMAP_ALLOC_ALL: c_uint = 1;

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
pub struct XExtData {
   pub number: c_int,
   pub next: *mut XExtData,
   pub free_private: Option<extern "C" fn(extension: *mut XExtData) -> c_int>,
   pub private_data: XPointer,
}
impl Clone for XExtData {
   fn clone(&self) -> Self { *self }
}
impl Default for XExtData {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Visual {
   pub ext_data: *mut XExtData,
   pub visualid: VisualID,
   pub class: c_int,
   pub red_mask: c_ulong,
   pub green_mask: c_ulong,
   pub blue_mask: c_ulong,
   pub bits_per_rgb: c_int,
   pub map_entries: c_int,
}
impl Clone for Visual {
   fn clone(&self) -> Self { *self }
}
impl Default for Visual {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Depth {
   pub depth: c_int,
   pub nvisuals: c_int,
   pub visuals: *mut Visual,
}
impl Clone for Depth {
   fn clone(&self) -> Self { *self }
}
impl Default for Depth {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Screen {
   pub ext_data: *mut XExtData,
   pub display: *mut XDisplay,
   pub root: Window,
   pub width: c_int,
   pub height: c_int,
   pub mwidth: c_int,
   pub mheight: c_int,
   pub ndepths: c_int,
   pub depths: *mut Depth,
   pub root_depth: c_int,
   pub root_visual: *mut Visual,
   pub default_gc: GC,
   pub cmap: Colormap,
   pub white_pixel: c_ulong,
   pub black_pixel: c_ulong,
   pub max_maps: c_int,
   pub min_maps: c_int,
   pub backing_store: c_int,
   pub save_unders: c_int,
   pub root_input_mask: c_long,
}
impl Clone for Screen {
   fn clone(&self) -> Self { *self }
}
impl Default for Screen {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ScreenFormat {
   pub ext_data: *mut XExtData,
   pub depth: c_int,
   pub bits_per_pixel: c_int,
   pub scanline_pad: c_int,
}
impl Clone for ScreenFormat {
   fn clone(&self) -> Self { *self }
}
impl Default for ScreenFormat {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct XPrivDisplay {
   pub ext_data: *mut XExtData,
   pub private1: *mut XPrivate,
   pub fd: c_int,
   pub private2: c_int,
   pub proto_major_version: c_int,
   pub proto_minor_version: c_int,
   pub vendor: *mut c_char,
   pub private3: XID,
   pub private4: XID,
   pub private5: XID,
   pub private6: c_int,
   pub resource_alloc: Option<extern "C" fn(arg1: *mut XDisplay) -> XID>,
   pub byte_order: c_int,
   pub bitmap_unit: c_int,
   pub bitmap_pad: c_int,
   pub bitmap_bit_order: c_int,
   pub nformats: c_int,
   pub pixmap_format: *mut ScreenFormat,
   pub private8: c_int,
   pub release: c_int,
   pub private9: *mut XPrivate,
   pub private10: *mut XPrivate,
   pub qlen: c_int,
   pub last_request_read: c_ulong,
   pub request: c_ulong,
   pub private11: XPointer,
   pub private12: XPointer,
   pub private13: XPointer,
   pub private14: XPointer,
   pub max_request_size: c_uint,
   pub db: *mut XrmHashBucketRec,
   pub private15: Option<extern "C" fn(arg1: *mut XDisplay) -> c_int>,
   pub display_name: *mut c_char,
   pub default_screen: c_int,
   pub nscreens: c_int,
   pub screens: *mut Screen,
   pub motion_buffer: c_ulong,
   pub private16: c_ulong,
   pub min_keycode: c_int,
   pub max_keycode: c_int,
   pub private17: XPointer,
   pub private18: XPointer,
   pub private19: c_int,
   pub xdefaults: *mut c_char,
}
impl Clone for XPrivDisplay {
   fn clone(&self) -> Self { *self }
}
impl Default for XPrivDisplay {
   fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type _XPrivDisplay = *mut XPrivDisplay;

#[repr(C)]
#[derive(Copy)]
pub struct xcb_screen_t {
   pub root: xcb_window_t,
   pub default_colormap: xcb_colormap_t,
   pub white_pixel: uint32_t,
   pub black_pixel: uint32_t,
   pub current_input_masks: uint32_t,
   pub width_in_pixels: uint16_t,
   pub height_in_pixels: uint16_t,
   pub width_in_millimeters: uint16_t,
   pub height_in_millimeters: uint16_t,
   pub min_installed_maps: uint16_t,
   pub max_installed_maps: uint16_t,
   pub root_visual: xcb_visualid_t,
   pub backing_stores: uint8_t,
   pub save_unders: uint8_t,
   pub root_depth: uint8_t,
   pub allowed_depths_len: uint8_t,
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
   pub status: uint8_t,
   pub pad0: uint8_t,
   pub protocol_major_version: uint16_t,
   pub protocol_minor_version: uint16_t,
   pub length: uint16_t,
   pub release_number: uint32_t,
   pub resource_id_base: uint32_t,
   pub resource_id_mask: uint32_t,
   pub motion_buffer_size: uint32_t,
   pub vendor_len: uint16_t,
   pub maximum_request_length: uint16_t,
   pub roots_len: uint8_t,
   pub pixmap_formats_len: uint8_t,
   pub image_byte_order: uint8_t,
   pub bitmap_format_bit_order: uint8_t,
   pub bitmap_format_scanline_unit: uint8_t,
   pub bitmap_format_scanline_pad: uint8_t,
   pub min_keycode: xcb_keycode_t,
   pub max_keycode: xcb_keycode_t,
   pub pad1: [uint8_t; 4usize],
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

#[link(name="X11")]
extern "C" {
   pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
   pub fn XCloseDisplay(display: *mut Display) -> c_int;
}

#[link(name="X11-xcb")]
extern "C" {
   pub fn XGetXCBConnection(display: *mut Display) -> *mut xcb_connection_t;
   pub fn XSetEventQueueOwner(
      display: *mut Display,
      owner: XEventQueueOwner
   ) -> ();
}

#[link(name="xcb")]
extern "C" {
   pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
   pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t) -> ();
   pub fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;

   pub fn xcb_setup_roots_iterator(
      R: *const xcb_setup_t
   ) -> xcb_screen_iterator_t;

   pub fn xcb_create_colormap(
      c: *mut xcb_connection_t,
      alloc: uint8_t,
      mid: xcb_colormap_t,
      window: xcb_window_t,
      visual: xcb_visualid_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_create_window(
      c: *mut xcb_connection_t,
      depth: uint8_t,
      wid: xcb_window_t,
      parent: xcb_window_t,
      x: int16_t,
      y: int16_t,
      width: uint16_t,
      height: uint16_t,
      border_width: uint16_t,
      _class: uint16_t,
      visual: xcb_visualid_t,
      value_mask: uint32_t,
      value_list: *const uint32_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_destroy_window(
      c: *mut xcb_connection_t,
      window: xcb_window_t
   ) -> xcb_void_cookie_t;

   pub fn xcb_map_window(
      c: *mut xcb_connection_t,
      window: xcb_window_t
   ) -> xcb_void_cookie_t;
}

#[link(name="GL")]
extern "C" {
}

#[macro_export]
macro_rules! DefaultScreen {
   ($display:expr) => (
      unsafe{
         (
            *($display as $crate::platform::x11::ffi::_XPrivDisplay)
         ).default_screen
      }
   )
}

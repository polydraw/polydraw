#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use libc::{c_char, c_uchar, c_ushort, c_int, c_uint, c_long, c_ulong};
use std::mem;

pub enum XDisplay { }
pub enum XPrivate { }
pub enum XrmHashBucketRec { }
pub enum XGC { }

pub enum xcb_connection_t { }

pub type uint8_t = c_uchar;
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

#[link(name="X11")]
extern "C" {
   pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
   pub fn XCloseDisplay(display: *mut Display) -> c_int;
}

#[link(name="X11-xcb")]
extern "C" {
   pub fn XGetXCBConnection(display: *mut Display) -> *mut xcb_connection_t;
   pub fn XSetEventQueueOwner(display: *mut Display, owner: XEventQueueOwner) -> ();
}

#[link(name="xcb")]
extern "C" {
   pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
   pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
   pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t) -> ();
}

#[macro_export]
macro_rules! DefaultScreen {
   ($display:expr) => (
      unsafe{
         (*($display as $crate::platform::x11::ffi::_XPrivDisplay)).default_screen
      }
   )
}

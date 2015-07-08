pub mod ffi {
   #![allow(non_upper_case_globals)]

   use libc::{
      c_char, c_int, c_uint, c_long, c_ulong, c_void
   };
   use std::mem;
   use ::os::xcb::ffi::xcb_connection_t;

   pub enum XDisplay { }
   pub enum XPrivate { }
   pub enum XrmHashBucketRec { }
   pub enum XGC { }

   pub type XID = c_ulong;
   pub type GC = *mut XGC;
   pub type Display = XDisplay;
   pub type Colormap = XID;
   pub type Window = XID;
   pub type XPointer = *mut c_char;
   pub type VisualID = c_ulong;

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

   #[link(name="X11")]
   extern "C" {
      pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
      pub fn XCloseDisplay(display: *mut Display) -> c_int;
      pub fn XFree(data: *mut c_void) -> c_int;
   }

   #[link(name="X11-xcb")]
   extern "C" {
      pub fn XGetXCBConnection(
         display: *mut Display
      ) -> *mut xcb_connection_t;

      pub fn XSetEventQueueOwner(
         display: *mut Display,
         owner: XEventQueueOwner
      ) -> ();
   }
}

use std::ptr;
use std::ffi::{CString, CStr};

use libc::c_char;

use ::error::{RuntimeError, ErrorKind};
use ::os::xcb::Connection;

pub struct Display {
   pub ptr: *mut ffi::Display
}

impl Display {
   pub fn default() -> Result<Self, RuntimeError> {
      Display::from_ptr(ptr::null())
   }

   pub fn new<T: Into<Vec<u8>>>(name: T) -> Result<Self, RuntimeError> {
      let c_name = try!(CString::new(name.into()));

      Display::from_ptr(c_name.as_ptr())
   }

   fn from_ptr(name: *const c_char) -> Result<Self, RuntimeError> {
      let display_ptr = unsafe {
         ffi::XOpenDisplay(name)
      };

      if display_ptr.is_null() {
         let description = format!(
            "Opening X display '{}' failed",
            unsafe { CStr::from_ptr(name).to_str().unwrap() }
         );
         return Err(RuntimeError::new(
            ErrorKind::Xlib,
            description
         ));
      }

      Ok(
         Display {
            ptr: display_ptr
         }
      )
   }

   pub fn xcb_connection(self: &Self) -> Result<Connection, RuntimeError> {
      let connection = unsafe {
         ffi::XGetXCBConnection(self.ptr)
      };
      if connection.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::Xlib,
            "Getting XCB connection from display failed".to_string()
         ));
      }

      Ok(
         Connection::new(connection)
      )
   }

   pub fn xcb_own_event_queue(self: &Self) {
      unsafe {
         ffi::XSetEventQueueOwner(
            self.ptr,
            ffi::XCBOwnsEventQueue
         )
      };
   }
}

impl Drop for Display {
   fn drop (&mut self) {
      unsafe {
         ffi::XCloseDisplay(self.ptr);
      }
   }
}

#[cfg(test)]
mod test {
   use std::env;
   use super::Display;

   #[test]
   fn test_create_display() {
      assert!(Display::default().is_ok());
   }

   #[test]
   fn test_create_display_from_env() {
      let name = env::var("DISPLAY").unwrap();
      assert!(Display::new(name).is_ok());
   }

   #[test]
   fn test_create_display_error_str() {
      assert!(Display::new("ErrorStringHere").is_err());
   }

   #[test]
   fn test_create_display_nul_str() {
      assert!(Display::new(&b"one\0two"[..]).is_err());
   }

   #[test]
   fn test_create_get_xcb_connection() {
      assert!(Display::default().unwrap().xcb_connection().is_ok());
   }
}


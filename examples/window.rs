#[macro_use]
extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::ffi;

fn main() {
   let display = unsafe { ffi::XOpenDisplay(ptr::null()) };
   if display.is_null() {
      println!("Can't open display");
      return;
   }

   let default_screen = DefaultScreen!(display);

   let connection = unsafe { ffi::XGetXCBConnection(display) };
   if connection.is_null() {
      unsafe { ffi::XCloseDisplay(display) };
      println!("Can't get xcb connection from display");
      return;
   }

   unsafe {
      ffi::XSetEventQueueOwner(display, ffi::XCBOwnsEventQueue)
   };

   println!("{:?}", default_screen);
   println!("{:?}", connection);

   unsafe { ffi::XCloseDisplay(display) };
}

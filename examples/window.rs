#[macro_use]
extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::ffi::{XOpenDisplay, XGetXCBConnection, XCloseDisplay};

fn main() {
   let display = unsafe { XOpenDisplay(ptr::null()) };
   if display.is_null() {
      println!("Can't open display");
      return;
   }

   let default_screen = DefaultScreen!(display);

   let connection = unsafe { XGetXCBConnection(display) };
   if connection.is_null() {
      unsafe { XCloseDisplay(display) };
      println!("Can't get xcb connection from display");
      return;
   }

   println!("{:?}", default_screen);
   println!("{:?}", connection);
}

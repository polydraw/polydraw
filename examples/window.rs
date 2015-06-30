#[macro_use]
extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::xlib::ffi::XOpenDisplay;

fn main() {
   let display = unsafe { XOpenDisplay(ptr::null()) };
   if display.is_null() {
      println!("Can't open display");
      return;
   }

   let default_screen = DefaultScreen!(display);

   println!("{:?}", default_screen)
}

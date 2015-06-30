extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::xlib::ffi::XOpenDisplay;

fn main() {
   let display = unsafe { XOpenDisplay(ptr::null()) };
   println!("{:?}", display)
}

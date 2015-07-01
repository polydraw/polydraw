#[macro_use]
extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::ffi;

fn screen_of_display(connection: *mut ffi::xcb_connection_t, screen: ffi::c_int) -> *mut ffi::xcb_screen_t {
   let mut iter = unsafe { ffi::xcb_setup_roots_iterator(ffi::xcb_get_setup(connection)) };

   let mut screen_num = screen;

   while screen_num > 0 && iter.rem != 0 {
      unsafe { ffi::xcb_screen_next(&mut iter) };
      screen_num -= 1;
   }

   iter.data
}


fn print_screen_info(screen: &ffi::xcb_screen_t) {
   println!("Informations of screen : {}", screen.root);
   println!("   width ............. : {}", screen.width_in_pixels);
   println!("   height ............ : {}", screen.height_in_pixels);
   println!("   white pixel ....... : {}", screen.white_pixel);
   println!("   black pixel ....... : {}", screen.black_pixel);
}

fn main() {
   let display = unsafe { ffi::XOpenDisplay(ptr::null()) };
   if display.is_null() {
      println!("Can't open display");
      return;
   }

   let connection = unsafe { ffi::XGetXCBConnection(display) };
   if connection.is_null() {
      unsafe { ffi::XCloseDisplay(display) };
      println!("Can't get xcb connection from display");
      return;
   }

   unsafe {
      ffi::XSetEventQueueOwner(display, ffi::XCBOwnsEventQueue)
   };

   let default_screen = DefaultScreen!(display);

   let screen = screen_of_display(connection, default_screen);

   unsafe { print_screen_info(&(*screen)) };

   unsafe { ffi::XCloseDisplay(display) };
}

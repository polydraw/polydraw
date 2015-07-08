extern crate polydraw;

use polydraw::os::x11::{Display};

fn main() {
   let display = Display::default().unwrap();

   println!("{:?}", display.ptr);

   let connection = display.xcb_connection().unwrap();

   println!("{:?}", connection.ptr);

   display.xcb_own_event_queue();
}

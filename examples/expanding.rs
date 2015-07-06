extern crate polydraw;

use polydraw::os::x11::{Display};

fn main() {
   let display = Display::default().unwrap();

   println!("{:?}", display.display_ptr);

   let connection = display.xcb_connection().unwrap();

   println!("{:?}", connection.connection_ptr);

   display.xcb_own_event_queue();
}

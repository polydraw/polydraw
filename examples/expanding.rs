extern crate polydraw;

use polydraw::os::x11::{Display};

fn main() {
   let display = match Display::new(":3") {
      Ok(display) => display,
      Err(err) => {
         panic!(err.description);
      }
   };

   println!("{:?}", display.display_ptr);
}

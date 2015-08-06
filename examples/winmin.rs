extern crate polydraw;

use polydraw::os::win;
use polydraw::os::wgl;

fn main() {
   win::register_window_class("WinMinClass");

   let window = win::Window::create(800, 600, "Win Min", "WinMinClass");
   let dc = window.dc();

   match wgl::init_pixel_format(dc) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   loop {
      let message = match win::Message::get() {
         Some(message) => message,
         None => break
      };

      message.translate();
      message.dispatch();
   }
}

extern crate polydraw;

use polydraw::os::win;

fn main() {
   win::register_window_class("WinMinClass");

   win::Window::create(800, 600, "Win Min", "WinMinClass");

   loop {
      let message = match win::Message::get() {
         Some(message) => message,
         None => break
      };

      message.translate();
      message.dispatch();
   }
}

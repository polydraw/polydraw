extern crate polydraw;

use polydraw::os::win;

fn main() {
   win::register_window_class("WinMinClass");

   win::create_window(800, 600, "Win Min", "WinMinClass");
}

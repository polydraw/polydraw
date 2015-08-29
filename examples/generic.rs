extern crate polydraw;

use polydraw::Application;

fn main() {
   let mut app = Application::new();

   app.window("PolyDraw").size(800, 400).position(50, 50).create().unwrap();

   app.run();
}

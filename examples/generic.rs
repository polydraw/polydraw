extern crate polydraw;

use polydraw::Application;

fn main() {
   let mut app = Application::new();

   let window = app.window("PolyDraw").size(800, 400).centered(false).create();

   app.run();
}

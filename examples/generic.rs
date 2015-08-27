extern crate polydraw;

use polydraw::Application;

fn main() {
   let app = Application::new();

   app.window("PolyDraw").size(800, 400).centered().create();

   app.run();
}

extern crate polydraw;

use polydraw::Application;

fn main() {
   Application::new()
      .title("Generic")
      .size(800, 400)
      .position(50, 50)
      .run();
}

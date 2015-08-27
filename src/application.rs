pub struct Application;

use window::WindowCreator;

impl Application {
   pub fn new() -> Self {
      Application
   }

   pub fn run(&self) {
   }

   pub fn window(&self, title: &str) -> WindowCreator {
      WindowCreator::new(title)
   }
}

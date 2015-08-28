pub struct Application;

use window::WindowCreator;

impl Application {
   pub fn new() -> Self {
      Application
   }

   pub fn run(&self) {
   }

   pub fn window<'a>(&'a mut self, title: &'a str) -> WindowCreator {
      WindowCreator::new(self, title)
   }
}

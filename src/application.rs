use error::RuntimeError;

#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OSApplication;

pub struct Application {
   pub os_application: OSApplication,
}

use window::{Window, WindowCreator};

impl Application {
   pub fn new() -> Self {
      Application {
         os_application: OSApplication::new().unwrap()
      }
   }

   pub fn run(&self) {
   }

   pub fn window<'a>(&'a mut self, title: &'a str) -> WindowCreator {
      WindowCreator::new(self, title)
   }

   pub fn _create_window(
      &self, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Window, RuntimeError> {

      let os_window = try!(self.os_application.create_os_window(
         title, x, y, width, height
      ));

      Ok(Window::new(os_window))
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.os_application.screen_size()
   }
}

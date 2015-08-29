#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OSApplication;

pub struct Application {
   os_application: OSApplication,
}

use window::WindowCreator;

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

   pub fn screen_size(&self) -> (u32, u32) {
      self.os_application.screen_size()
   }
}

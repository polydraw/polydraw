use error::RuntimeError;

use super::desktop::LinuxDesktop;
use super::initializer::LinuxInitializer;

pub struct LinuxApplication {
   desktop: LinuxDesktop,
   initializer: LinuxInitializer,
}

impl LinuxApplication {
   pub fn new(
      desktop: LinuxDesktop,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let initializer = try!(LinuxInitializer::new(
         &desktop, title, x, y, width, height,
      ));

      Ok(LinuxApplication {
         desktop: desktop,
         initializer: initializer,
      })
   }

   pub fn run(&self) {

   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.desktop.screen_size()
   }
}

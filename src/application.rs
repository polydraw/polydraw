use error::RuntimeError;

#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OsApplication;

#[cfg(target_os = "linux")]
pub use os::linux::desktop::LinuxDesktop as OsDesktop;

pub struct Application {
   pub os_application: OsApplication,
}

use super::creator::ApplicationCreator;

impl Application {
   pub fn new<'a>() -> ApplicationCreator<'a> {
      let desktop = match OsDesktop::new() {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      ApplicationCreator::new(desktop)
   }

   pub fn create(
      dekstop: OsDesktop,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Self {

      let os_application = match OsApplication::new(
         dekstop, title, x, y, width, height
      ) {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      Application {
         os_application: os_application
      }
   }

   pub fn run(&self) {
      self.os_application.run()
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.os_application.screen_size()
   }
}

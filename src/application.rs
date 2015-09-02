#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OsApplication;

#[cfg(target_os = "linux")]
pub use os::linux::desktop::LinuxDesktop as OsDesktop;

use super::frame::RenderFrame;

pub struct Application {
   pub os_application: OsApplication,
   pub render_frame: RenderFrame,
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
      desktop: OsDesktop,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Self {
      let (screen_width, screen_height) = desktop.screen_size();

      let render_frame = RenderFrame::new(width, height, screen_width, screen_height);

      let os_application = match OsApplication::new(
         desktop, title, x, y, width, height
      ) {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      Application {
         os_application: os_application,
         render_frame: render_frame,
      }
   }

   pub fn run(&mut self) {
      match self.os_application.run(&mut self.render_frame) {
         Ok(_) => {},
         Err(e) => {
            panic!(e.description);
         }
      }
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.os_application.screen_size()
   }
}

#[cfg(target_os = "windows")]
pub use os::windows::application::WindowsApplication as OsApplication;
#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OsApplication;

#[cfg(target_os = "windows")]
pub use os::windows::display::WindowsDisplay as OsDisplay;
#[cfg(target_os = "linux")]
pub use os::linux::display::LinuxDisplay as OsDisplay;

use super::renderer::Renderer;
use super::event_loop::EventLoop;

pub struct Application {
   pub os_application: OsApplication,
   pub initial_width: u32,
   pub initial_height: u32,
}

use super::creator::ApplicationCreator;

impl Application {
   pub fn new<'a>() -> ApplicationCreator<'a> {
      let display = match OsDisplay::new() {
         Ok(display) => display,
         Err(e) => {
            panic!(e.description);
         }
      };

      ApplicationCreator::new(display)
   }

   pub fn create(
      display: OsDisplay,
      title: &str,
      x: i32, y: i32,
      width: u32, height: u32
   ) -> Self {
      let os_application = match OsApplication::new(
         display, title, x, y, width, height
      ) {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      Application {
         os_application: os_application,
         initial_width: width,
         initial_height: height,
      }
   }

   pub fn run(&mut self, renderer: &mut Renderer) {
      let event_loop = EventLoop::new(&self.os_application);

      match event_loop.run(renderer, self.initial_width, self.initial_height) {
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

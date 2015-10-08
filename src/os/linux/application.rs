use error::RuntimeError;

use super::display::LinuxDisplay;
use super::window::{LinuxWindow, PollEventsIterator};
use super::egl_context::EglContext;

pub struct LinuxApplication {
   display: LinuxDisplay,
   window: LinuxWindow,
   egl: EglContext,
}

impl LinuxApplication {
   pub fn new(
      display: LinuxDisplay,
      title: &str,
      x: i32, y: i32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(LinuxWindow::new(
         &display, title, x, y, width, height,
      ));

      let egl = try!(EglContext::new(&display.display, &window.window));

      Ok(LinuxApplication {
         display: display,
         window: window,
         egl: egl,
      })
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.display.screen_size()
   }

   #[inline]
   pub fn poll_events(&self) -> PollEventsIterator {
      self.window.poll_events()
   }

   #[inline]
   pub fn swap_buffers(&self) -> Result<(), RuntimeError> {
      self.egl.swap_buffers()
   }
}

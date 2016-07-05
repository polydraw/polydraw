use error::{RuntimeError, VoidResult};

use super::display::LinuxDisplay;
use super::window::{LinuxWindow, PollEventsIterator};
use super::context::{Context, create_context};

pub struct LinuxApplication {
   display: LinuxDisplay,
   window: LinuxWindow,
   context: Box<Context>,
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

      let context = try!(
         create_context(&display.display, &display.screen_id, &window.window)
      );

      Ok(LinuxApplication {
         display: display,
         window: window,
         context: context,
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
   pub fn swap_buffers(&self) -> VoidResult {
      self.context.swap_buffers()
   }
}

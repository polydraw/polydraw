use error::{RuntimeError, VoidResult};
use frame::GPUFrame;

use sys::wgl;

use super::display::WindowsDisplay;
use super::window::{WindowsWindow, PollEventsIterator};
use super::wgl_context::WglContext;

pub struct WindowsApplication {
   display: WindowsDisplay,
   window: WindowsWindow,
   wgl: WglContext,
}

impl WindowsApplication {
   pub fn new(
      display: WindowsDisplay,
      title: &str,
      x: i32, y: i32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(WindowsWindow::new(title, x, y, width, height));

      let wgl = try!(WglContext::new(&window.device_context));

      Ok(WindowsApplication {
         display: display,
         window: window,
         wgl: wgl,
      })
   }

   #[inline]
   pub fn create_gpu_frame(
      &self, width: u32, height: u32
   ) -> Result<Box<GPUFrame>, RuntimeError> {
      self.wgl.create_gpu_frame(width, height)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      self.display.screen_size()
   }

   #[inline]
   pub fn poll_events(&self) -> PollEventsIterator {
      self.window.poll_events()
   }

   #[inline]
   pub fn swap_buffers(&self) -> VoidResult {
      wgl::swap_buffers(&self.window.device_context);
      Ok(())
   }
}

use error::RuntimeError;

use frame::RenderFrame;
use renderer::Renderer;

use super::display::WindowsDisplay;
use super::window::WindowsWindow;
use super::event_loop::WindowsEventLoop;

pub struct WindowsApplication {
   display: WindowsDisplay,
   window: WindowsWindow,
}

impl WindowsApplication {
   pub fn new(
      display: WindowsDisplay,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(WindowsWindow::new(title, x, y, width, height));

      Ok(WindowsApplication {
         display: display,
         window: window,
      })
   }

   pub fn run(
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {

      let mut event_loop = WindowsEventLoop::new(
         renderer,
         render_frame,
         &self.window.device_context,
         &self.window.gl.texture,
         &self.window.gl.framebuffer,
      );

      event_loop.run()
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.display.screen_size()
   }
}

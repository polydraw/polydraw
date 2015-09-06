use error::RuntimeError;

use frame::RenderFrame;
use renderer::Renderer;

use super::display::LinuxDisplay;
use super::window::LinuxWindow;
use super::event_loop::LinuxEventLoop;

pub struct LinuxApplication {
   display: LinuxDisplay,
   window: LinuxWindow,
}

impl LinuxApplication {
   pub fn new(
      display: LinuxDisplay,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(LinuxWindow::new(
         &display, title, x, y, width, height,
      ));

      Ok(LinuxApplication {
         display: display,
         window: window,
      })
   }

   pub fn run(
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {

      let mut event_loop = LinuxEventLoop::new(
         renderer,
         render_frame,
         &self.display.connection,
         &self.window.window,
         &self.window.atoms,
         &self.window.gl.texture,
         &self.window.gl.framebuffer,
         &self.window.egl.display,
         &self.window.egl.surface,
      );

      event_loop.run()
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.display.screen_size()
   }
}

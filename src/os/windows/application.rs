use error::RuntimeError;

use frame::RenderFrame;
use renderer::Renderer;

use super::desktop::WindowsDesktop;
use super::initializer::WindowsInitializer;
use super::event_loop::WindowsEventLoop;

pub struct WindowsApplication {
   desktop: WindowsDesktop,
   initializer: WindowsInitializer,
}

impl WindowsApplication {
   pub fn new(
      desktop: WindowsDesktop,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let initializer = try!(WindowsInitializer::new(
         &desktop, title, x, y, width, height,
      ));

      Ok(WindowsApplication {
         desktop: desktop,
         initializer: initializer,
      })
   }

   pub fn run(
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {

      let mut event_loop = WindowsEventLoop::new(
         renderer,
         render_frame,
         &self.initializer.device_context,
         &self.initializer.gl.texture,
         &self.initializer.gl.framebuffer,
      );

      event_loop.run()
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.desktop.screen_size()
   }
}

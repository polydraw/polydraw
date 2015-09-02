use error::RuntimeError;

use frame::RenderFrame;

use super::desktop::LinuxDesktop;
use super::initializer::LinuxInitializer;
use super::event_loop::LinuxEventLoop;

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

   pub fn run(&self, render_frame: &mut RenderFrame) -> Result<(), RuntimeError> {
      let mut event_loop = LinuxEventLoop::new(
         render_frame,
         &self.desktop.connection,
         &self.initializer.window,
         &self.initializer.atoms,
         &self.initializer.gl.texture,
         &self.initializer.gl.framebuffer,
         &self.initializer.egl.display,
         &self.initializer.egl.surface,
      );

      event_loop.run()
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.desktop.screen_size()
   }
}

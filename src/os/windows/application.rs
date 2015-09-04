use error::RuntimeError;

use frame::RenderFrame;
use renderer::Renderer;

use super::desktop::WindowsDesktop;

pub struct WindowsApplication {
   desktop: WindowsDesktop,
}

impl WindowsApplication {
   pub fn new(
      desktop: WindowsDesktop,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      Ok(WindowsApplication {
         desktop: desktop,
      })
   }

   pub fn run(
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {

      Ok(())
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.desktop.screen_size()
   }
}

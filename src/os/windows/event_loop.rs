use error::RuntimeError;

use sys::win32;
use sys::gl;
use sys::wgl;

use frame::RenderFrame;
use renderer::Renderer;


pub struct WindowsEventLoop<'a> {
   renderer: &'a mut Renderer,
   render_frame: &'a mut RenderFrame,
   device_context: &'a win32::DeviceContext,
   texture: &'a gl::Texture,
   framebuffer: &'a gl::Framebuffer,
}

impl<'a> WindowsEventLoop<'a> {
   pub fn new(
      renderer: &'a mut Renderer,
      render_frame: &'a mut RenderFrame,
      device_context: &'a win32::DeviceContext,
      texture: &'a gl::Texture,
      framebuffer: &'a gl::Framebuffer,
   ) -> Self {
      WindowsEventLoop {
         renderer: renderer,
         render_frame: render_frame,
         device_context: device_context,
         texture: texture,
         framebuffer: framebuffer,
      }
   }

   pub fn run(&mut self) -> Result<(), RuntimeError> {
      loop {
         self.renderer.render(self.render_frame);

         self.texture.update(self.render_frame.width, self.render_frame.height, &self.render_frame.data);

         self.framebuffer.blit(self.render_frame.width, self.render_frame.height);

         gl::flush();

         wgl::swap_buffers(&self.device_context);

         if false {
            break;
         }
      }

      Ok(())
   }
}

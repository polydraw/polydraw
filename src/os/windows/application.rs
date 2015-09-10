use error::RuntimeError;

use sys::wgl;

use frame::RenderFrame;
use renderer::Renderer;
use event::Event;

use super::super::common::GlContext;

use super::display::WindowsDisplay;
use super::window::WindowsWindow;
use super::wgl_context::WglContext;

pub struct WindowsApplication {
   display: WindowsDisplay,
   window: WindowsWindow,
   #[allow(dead_code)] wgl: WglContext,
   gl: GlContext,
}

impl WindowsApplication {
   pub fn new(
      display: WindowsDisplay,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(WindowsWindow::new(title, x, y, width, height));

      let wgl = try!(WglContext::new(&window.device_context));

      let gl = try!(GlContext::new(width, height));

      Ok(WindowsApplication {
         display: display,
         window: window,
         wgl: wgl,
         gl: gl,
      })
   }

   pub fn run(
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {
      self.gl.framebuffer.bind();

      let mut quit = false;

      loop {
         for event in self.window.poll_events() {
            match event {
               Event::Resize(width, height) => {
                  render_frame.width = width;
                  render_frame.height = height;

                  self.gl.texture.resize(render_frame.width, render_frame.height);
               },
               Event::Quit => {
                  quit = true;
                  break
               },
               _ => {}
            }
         }

         if quit {
            break
         }

         renderer.render(render_frame);

         self.gl.texture.update(render_frame.width, render_frame.height, &render_frame.data);

         self.gl.framebuffer.blit(render_frame.width, render_frame.height);

         wgl::swap_buffers(&self.window.device_context);
      }

      Ok(())
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.display.screen_size()
   }
}

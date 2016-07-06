use error::{RuntimeError, VoidResult};
use frame::GPUFrame;

use sys::x11;
use sys::xcb;
use sys::glx;
use sys::gl;

use super::Context;

pub struct GlxContext {
   pub display: glx::Display,
   pub version: glx::Version,
   pub config: glx::Config,
   pub context: glx::Context,
   pub rendering_area: glx::GLXWindow,
}

impl Context for GlxContext {
   fn new(x11_display: &x11::Display, screen_id: &x11::ScreenID, window: &xcb::Window) -> Result<Self, RuntimeError> {
      let display = glx::Display{
         ptr: x11_display.ptr,
      };

      let version = try!(glx::initialize(&display));

      let config = try!(glx::choose_config(&display, screen_id));

      let context = try!(glx::create_new_context(&display, &config));

      let rendering_area = try!(Self::init_rendering_area(&display, &config, &context, window));

      try!(Self::init_gl());

      Ok(GlxContext {
         display: display,
         version: version,
         config: config,
         context: context,
         rendering_area: rendering_area,
      })
   }

   #[inline]
   fn create_gpu_frame(
      &self, width: u32, height: u32
   ) -> Result<Box<GPUFrame>, RuntimeError> {

      match gl::gl_frame::GLFrame::new(width, height) {
         Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
         Err(e) => Err(e)
      }
   }

   #[inline]
   fn swap_buffers(&self) -> VoidResult {
      glx::swap_buffers(&self.display, self.rendering_area)
   }
}

impl GlxContext {
   #[inline]
   pub fn init_rendering_area(
      display: &glx::Display,
      config: &glx::Config,
      context: &glx::Context,
      window: &xcb::Window
   ) -> Result<glx::GLXWindow, RuntimeError> {
      let rendering_area = try!(glx::create_rendering_area(&display, &config, window.window_id.id));

      try!(glx::make_current(&display, rendering_area, &context));

      // TODO
      // try!(glx::swap_interval(display, 0));

      Ok(rendering_area)
   }

   #[inline]
   pub fn init_gl() -> VoidResult {
      gl::load(&glx::Loader::new());

      try!(gl::reset_pixelstore_alignment());
      try!(gl::enable_framebuffer_srgb());

      Ok(())
   }
}

use error::{RuntimeError, VoidResult};
use frame::GPUFrame;

use sys::x11;
use sys::xcb;
use sys::egl;
use sys::gl;

use super::Context;

pub struct EglContext {
   pub display: egl::Display,
   pub version: egl::Version,
   pub config: egl::Config,
   pub context: egl::Context,
   pub surface: egl::Surface,
}

impl Context for EglContext {
   fn new(x11_display: &x11::Display, _: &x11::ScreenID, window: &xcb::Window) -> Result<Self, RuntimeError> {
      try!(Self::bind());

      let display = try!(egl::Display::from_native(x11_display));

      let version = try!(display.initialize_egl());

      let config = try!(display.choose_config());

      let context = try!(Self::init_context(&display, &config));

      let surface = try!(Self::init_surface(&display, &config, &context, window));

      try!(Self::init_gl());

      Ok(EglContext {
         display: display,
         version: version,
         config: config,
         context: context,
         surface: surface,
      })
   }

   #[inline]
   fn create_gpu_frame(
      &self, width: u32, height: u32
   ) -> Result<Box<GPUFrame>, RuntimeError> {

      match gl::frame::buffer::BufferFrame::new(width, height) {
         Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
         Err(e) => Err(e)
      }
   }

   #[inline]
   fn swap_buffers(&self) -> VoidResult {
      self.display.swap_buffers(&self.surface)
   }
}

impl EglContext {
   #[inline]
   pub fn bind() -> VoidResult {
      egl::bind_api(egl::API::OpenGL)
   }

   #[inline]
   pub fn init_context(
      display: &egl::Display, config: &egl::Config
   ) -> Result<egl::Context, RuntimeError> {

      let context = try!(display.create_context(config));

      try!(display.query_context(&context));

      Ok(context)
   }

   #[inline]
   pub fn init_surface(
      display: &egl::Display,
      config: &egl::Config,
      context: &egl::Context,
      window: &xcb::Window
   ) -> Result<egl::Surface, RuntimeError> {

      let surface = try!(display.create_window_surface(
         config,
         &window.window_id.id
      ));

      try!(display.make_current(
         &surface,
         &surface,
         context
      ));

      try!(display.swap_interval(0));

      Ok(surface)
   }

   #[inline]
   pub fn init_gl() -> VoidResult {
      gl::load(&egl::Loader::new());

      try!(gl::reset_pixelstore_alignment());
      try!(gl::enable_framebuffer_srgb());

      Ok(())
   }
}

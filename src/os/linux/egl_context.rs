use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::egl;
use sys::gl;

pub struct EglContext {
   pub display: egl::Display,
   pub version: egl::Version,
   pub config: egl::Config,
   pub context: egl::Context,
   pub surface: egl::Surface,
}

impl EglContext {
   pub fn new(x11_display: &x11::Display, window: &xcb::Window) -> Result<Self, RuntimeError> {
      try!(Self::bind());

      let display = try!(egl::Display::from_native(x11_display));

      let version = try!(egl::initialize(&display));

      let config = try!(egl::choose_config(&display));

      let context = try!(Self::init_context(&display, &config));

      let surface = try!(Self::init_surface(&display, &config, &context, window));

      Self::init_gl();

      Ok(EglContext {
         display: display,
         version: version,
         config: config,
         context: context,
         surface: surface,
      })
   }

   #[inline]
   pub fn bind() -> Result<(), RuntimeError> {
      egl::bind_api(egl::API::OpenGL)
   }

   #[inline]
   pub fn init_context(
      display: &egl::Display, config: &egl::Config
   ) -> Result<egl::Context, RuntimeError> {

      let context = try!(egl::create_context(&display, &config));

      try!(egl::query_context(&display, &context));

      Ok(context)
   }

   #[inline]
   pub fn init_surface(
      display: &egl::Display,
      config: &egl::Config,
      context: &egl::Context,
      window: &xcb::Window
   ) -> Result<egl::Surface, RuntimeError> {

      let surface = try!(egl::create_window_surface(
         display,
         config,
         &window.window_id.id
      ));

      try!(egl::make_current(
         display,
         &surface,
         &surface,
         context
      ));

      try!(egl::swap_interval(display, 0));

      Ok(surface)
   }

   #[inline]
   pub fn init_gl() {
      gl::load(&egl::Loader::new());
      gl::reset_pixelstore_alignment();
   }

   #[inline]
   pub fn swap_buffers(&self) -> Result<(), RuntimeError> {
      egl::swap_buffers(&self.display, &self.surface)
   }
}

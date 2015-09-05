use std::rc::Rc;

use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::egl;
use sys::gl;

use super::desktop::LinuxDesktop;
use super::super::common::GlInitializer;

pub struct XcbAtoms {
   pub protocols_atom: xcb::Atom,
   pub delete_window_atom: xcb::Atom,
}

pub struct LinuxInitializer {
   pub window: xcb::Window,
   pub atoms: XcbAtoms,
   pub egl: EglInitializer,
   pub gl: GlInitializer,
}

impl LinuxInitializer {
   pub fn new(
      desktop: &LinuxDesktop, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(Self::init_window(
         &desktop.connection, &desktop.screen,
         title, x, y, width, height
      ));

      let atoms = try!(Self::init_atoms(&window));

      let egl = try!(EglInitializer::new(&desktop.display, &window));

      let gl = try!(GlInitializer::new(width, height));

      Ok(LinuxInitializer {
         window: window,
         atoms: atoms,
         egl: egl,
         gl: gl,
      })
   }

   #[inline]
   pub fn init_window(
      connection: &Rc<xcb::Connection>, screen: &xcb::Screen,
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<xcb::Window, RuntimeError> {

      let window = try!(xcb::Window::create(
         connection, &screen, width, height,
      ));

      try!(window.set_title(title));

      try!(window.map());

      try!(window.position(x, y));

      Ok(window)
   }

   #[inline]
   pub fn init_atoms(window: &xcb::Window) -> Result<XcbAtoms, RuntimeError> {
      let (protocols_atom, delete_window_atom) = try!(window.register_close_event());

      Ok(XcbAtoms {
         protocols_atom: protocols_atom,
         delete_window_atom: delete_window_atom,
      })
   }
}

pub struct EglInitializer {
   pub display: egl::Display,
   pub version: egl::Version,
   pub config: egl::Config,
   pub context: egl::Context,
   pub surface: egl::Surface,
}

impl EglInitializer {
   pub fn new(x11_display: &x11::Display, window: &xcb::Window) -> Result<Self, RuntimeError> {
      try!(Self::bind());

      let display = try!(egl::Display::from_native(x11_display));

      let version = try!(egl::initialize(&display));

      let config = try!(egl::choose_config(&display));

      let context = try!(Self::init_context(&display, &config));

      let surface = try!(Self::init_surface(&display, &config, &context, window));

      Self::init_gl();

      Ok(EglInitializer {
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
      gl::load(egl::Loader::new());
      gl::reset_pixelstore_alignment();
   }
}

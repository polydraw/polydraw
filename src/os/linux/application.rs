use std::rc::Rc;

use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::egl;
use sys::gl;

use super::window::LinuxWindow;

pub struct LinuxApplication {
   pub x11_handler: X11Handler,
   pub connection: ConnectionHandler,
   pub screen: ScreenHandler,
   pub egl_handler: EglHandler,
}

impl LinuxApplication {
   pub fn new() -> Result<Self, RuntimeError> {
      let x11_handler = try!(X11Handler::new());
      let connection = try!(ConnectionHandler::new(&x11_handler));
      let screen = try!(ScreenHandler::new(&x11_handler, &connection));
      let egl_handler = try!(EglHandler::new(&x11_handler));

      gl::load(egl::Loader::new());
      gl::reset_pixelstore_alignment();

      Ok(LinuxApplication {
         x11_handler: x11_handler,
         connection: connection,
         screen: screen,
         egl_handler: egl_handler,
      })
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      self.screen.size()
   }

   pub fn create_os_window(
      &self, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<LinuxWindow, RuntimeError> {
      let xcb_window = try!(self.screen.create_window(
         &self.connection, x, y, width, height
      ));

      let surface = try!(self.egl_handler.create_surface(&xcb_window));

      try!(self.egl_handler.make_current(&surface));

      try!(self.egl_handler.reset_swap_interval());

      let texture = gl::Texture::new(width, height);

      let framebuffer = gl::Framebuffer::new(&texture);

      Ok(LinuxWindow::new(xcb_window, surface, texture, framebuffer, title))
   }
}

pub struct X11Handler {
   pub display: x11::Display,
}

impl X11Handler {
   #[inline]
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(x11::Display::default());

      display.xcb_own_event_queue();

      Ok(X11Handler {
         display: display
      })
   }

   #[inline]
   pub fn connection(&self) -> Result<xcb::Connection, RuntimeError> {
      self.display.xcb_connection()
   }

   #[inline]
   pub fn screen_id(&self) -> x11::ScreenID {
      self.display.default_screen()
   }
}

pub struct ConnectionHandler {
   pub connection: Rc<xcb::Connection>,
}

impl ConnectionHandler {
   #[inline]
   pub fn new(x11_handler: &X11Handler) -> Result<Self, RuntimeError> {
      Ok(ConnectionHandler {
         connection: Rc::new(try!(x11_handler.connection()))
      })
   }

   #[inline]
   pub fn screen_of_display(&self, x11_handler: &X11Handler) -> Result<xcb::Screen, RuntimeError> {
      let screen_id = x11_handler.screen_id();

      self.connection.screen_of_display(&screen_id)
   }
}

pub struct ScreenHandler {
   pub screen: xcb::Screen,
}

impl ScreenHandler {
   #[inline]
   pub fn new(x11_handler: &X11Handler, connection: &ConnectionHandler) -> Result<Self, RuntimeError> {
      Ok(ScreenHandler {
         screen: try!(connection.screen_of_display(x11_handler))
      })
   }

   #[inline]
   pub fn size(&self) -> (u32, u32) {
      (
         self.screen.width_in_pixels() as u32,
         self.screen.height_in_pixels() as u32
      )
   }

   #[inline]
   pub fn create_window(
      &self, connection: &ConnectionHandler, x: u32, y: u32, width: u32, height: u32
   ) -> Result<xcb::Window, RuntimeError> {

      xcb::Window::create(
         &connection.connection, &self.screen, x, y, width, height,
      )
   }
}

pub struct EglHandler {
   pub display: egl::Display,
   pub version: egl::Version,
   pub config: egl::Config,
   pub context: egl::Context,
}

impl EglHandler {
   pub fn new(x11_handler: &X11Handler) -> Result<Self, RuntimeError> {
      try!(egl::bind_api(egl::API::OpenGL));

      let display = try!(egl::Display::from_native(&x11_handler.display));
      let version = try!(egl::initialize(&display));
      let config = try!(egl::choose_config(&display));
      let context = try!(egl::create_context(&display, &config));

      try!(egl::query_context(&display, &context));

      Ok(EglHandler {
         display: display,
         version: version,
         config: config,
         context: context,
      })
   }

   #[inline]
   pub fn create_surface(&self, xcb_window: &xcb::Window) -> Result<egl::Surface, RuntimeError> {
      egl::create_window_surface(
         &self.display,
         &self.config,
         &xcb_window.window_id.id
      )
   }

   #[inline]
   pub fn make_current(&self, surface: &egl::Surface) -> Result<(), RuntimeError> {
      egl::make_current(
         &self.display,
         surface,
         surface,
         &self.context
      )
   }

   #[inline]
   pub fn reset_swap_interval(&self) -> Result<(), RuntimeError> {
      egl::swap_interval(&self.display, 0)
   }
}

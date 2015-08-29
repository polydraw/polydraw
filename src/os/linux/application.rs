use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxApplication {
   pub x11_display_handler: X11DisplayHandler,
   pub connection_handler: ConnectionHandler,
}

impl LinuxApplication {
   pub fn new() -> Result<Self, RuntimeError> {
      let x11_display_handler = try!(X11DisplayHandler::new());
      let connection_handler = try!(ConnectionHandler::new(&x11_display_handler));

      Ok(LinuxApplication {
         x11_display_handler: x11_display_handler,
         connection_handler: connection_handler,
      })
   }

   pub fn desktop_size(&self) -> (u32, u32) {
      (1200, 800)
   }
}

pub struct X11DisplayHandler {
   pub display: x11::Display,
}

impl X11DisplayHandler {
   #[inline]
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(x11::Display::default());

      display.xcb_own_event_queue();

      Ok(X11DisplayHandler {
         display: display
      })
   }

   #[inline]
   pub fn connection(&self) -> Result<xcb::Connection, RuntimeError> {
      self.display.xcb_connection()
   }
}

pub struct ConnectionHandler {
   pub connection: xcb::Connection,
}

impl ConnectionHandler {
   #[inline]
   pub fn new(display_handler: &X11DisplayHandler) -> Result<Self, RuntimeError> {
      Ok(ConnectionHandler {
         connection: try!(display_handler.connection())
      })
   }
}

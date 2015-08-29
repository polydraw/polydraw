use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxApplication {
   pub x11_display: X11DisplayHandler,
   pub connection: ConnectionHandler,
   pub screen: ScreenHandler,
}

impl LinuxApplication {
   pub fn new() -> Result<Self, RuntimeError> {
      let x11_display = try!(X11DisplayHandler::new());
      let connection = try!(ConnectionHandler::new(&x11_display));
      let screen = ScreenHandler::new(&x11_display, &connection);

      Ok(LinuxApplication {
         x11_display: x11_display,
         connection: connection,
         screen: screen,
      })
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.screen.size()
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

   #[inline]
   pub fn screen_id(&self) -> x11::ScreenID {
      self.display.default_screen()
   }
}

pub struct ConnectionHandler {
   pub connection: xcb::Connection,
}

impl ConnectionHandler {
   #[inline]
   pub fn new(display: &X11DisplayHandler) -> Result<Self, RuntimeError> {
      Ok(ConnectionHandler {
         connection: try!(display.connection())
      })
   }

   #[inline]
   pub fn screen_of_display(&self, display: &X11DisplayHandler) -> xcb::Screen {
      let screen_id = display.screen_id();

      self.connection.screen_of_display(&screen_id)
   }
}

pub struct ScreenHandler {
   pub screen: xcb::Screen,
}

impl ScreenHandler {
   #[inline]
   pub fn new(display: &X11DisplayHandler, connection: &ConnectionHandler) -> Self {
      ScreenHandler {
         screen: connection.screen_of_display(display)
      }
   }

   pub fn size(&self) -> (u32, u32) {
      (
         self.screen.width_in_pixels() as u32,
         self.screen.height_in_pixels() as u32
      )
   }
}

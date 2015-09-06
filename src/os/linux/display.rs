use std::rc::Rc;

use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxDisplay {
   pub display: x11::Display,
   pub connection: Rc<xcb::Connection>,
   pub screen: xcb::Screen,
}

impl LinuxDisplay {
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(Self::init_display());

      let connection = Rc::new(try!(display.xcb_connection()));

      let screen = try!(Self::init_screen(&display, &connection));

      Ok(LinuxDisplay {
         display: display,
         connection: connection,
         screen: screen,
      })
   }

   #[inline]
   pub fn init_display() -> Result<x11::Display, RuntimeError> {
      let display = try!(x11::Display::default());

      display.xcb_own_event_queue();

      Ok(display)
   }

   #[inline]
   pub fn init_screen(
      display: &x11::Display, connection: &xcb::Connection
   ) -> Result<xcb::Screen, RuntimeError> {

      let screen_id = display.default_screen();

      connection.screen_of_display(&screen_id)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      (
         self.screen.width_in_pixels() as u32,
         self.screen.height_in_pixels() as u32
      )
   }
}

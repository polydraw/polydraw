use std::rc::Rc;

use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxDisplay {
   pub display: x11::Display,
   pub connection: Rc<xcb::Connection>,
   pub screen: xcb::Screen,
   pub screen_id: x11::ScreenID,
}

impl LinuxDisplay {
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(Self::init_display());

      let connection = Rc::new(try!(display.xcb_connection()));

      let screen_id = display.default_screen();

      let screen = try!(connection.screen_of_display(&screen_id));

      Ok(LinuxDisplay {
         display: display,
         connection: connection,
         screen: screen,
         screen_id: screen_id,
      })
   }

   #[inline]
   pub fn init_display() -> Result<x11::Display, RuntimeError> {
      let display = try!(x11::Display::default());

      display.xcb_own_event_queue();

      Ok(display)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      (
         self.screen.width_in_pixels(),
         self.screen.height_in_pixels()
      )
   }
}

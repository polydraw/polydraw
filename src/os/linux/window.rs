use error::{RuntimeError, ErrorKind};

use sys::x11;
use sys::xcb;
use sys::egl;

pub struct LinuxWindow {
   display: x11::Display,
   connection: xcb::Connection,
}

impl LinuxWindow {
   pub fn new(title: &str) -> Result<Self, RuntimeError> {
      let display = match x11::Display::default() {
         Ok(display) => display,
         Err(e) => return Err(e)
      };

      let connection = match display.xcb_connection() {
         Ok(connection) => connection,
         Err(e) => return Err(e)
      };

      Ok(LinuxWindow {
         display: display,
         connection: connection,
      })
   }
}

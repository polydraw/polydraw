use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxWindow {
   pub display: x11::Display,
   pub connection: xcb::Connection,
}

impl LinuxWindow {
   #[allow(unused_variables)]
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

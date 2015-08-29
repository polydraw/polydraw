use error::RuntimeError;

use sys::x11;
use sys::xcb;

pub struct LinuxApplication {
   pub display: x11::Display,
   pub connection: xcb::Connection,
}

impl LinuxApplication {
   #[allow(unused_variables)]
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(Self::create_display());

      println!("X11 display ............... : {:?}", display.ptr);

      let connection = match display.xcb_connection() {
         Ok(connection) => connection,
         Err(e) => return Err(e)
      };

      Ok(LinuxApplication {
         display: display,
         connection: connection,
      })
   }

   pub fn desktop_size(&self) -> (u32, u32) {
      (1200, 800)
   }
}

trait X11DisplayHandler {
   fn create_display() -> Result<x11::Display, RuntimeError>;
}

impl X11DisplayHandler for LinuxApplication {
   #[inline]
   fn create_display() -> Result<x11::Display, RuntimeError> {
      x11::Display::default()
   }
}

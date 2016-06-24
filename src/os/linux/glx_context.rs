use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::glx;

pub struct GlxContext {
}

impl GlxContext {
   pub fn new(x11_display: &x11::Display, screen_id: &x11::ScreenID, window: &xcb::Window) -> Result<Self, RuntimeError> {
      let display = glx::Display{
         ptr: x11_display.ptr,
      };

      let version = try!(glx::initialize(&display));

      let config = try!(glx::choose_config(&display, screen_id));

      println!("GLX Version {}.{}", version.major, version.minor);

      Ok(GlxContext {
      })
   }
}

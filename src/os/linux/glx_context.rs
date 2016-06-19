use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::glx;

pub struct GlxContext {
}

impl GlxContext {
   pub fn new(x11_display: &x11::Display, _: &xcb::Window) -> Result<Self, RuntimeError> {
      let display = glx::Display{
         ptr: x11_display.ptr,
      };

      let version = try!(glx::initialize(&display));

      println!("GLX Version {}.{}", version.major, version.minor);

      Ok(GlxContext {
      })
   }
}

use error::RuntimeError;

use sys::x11;
use sys::glx;

pub struct GlxContext {
   pub version: glx::Version,
   pub config: glx::Config,
}

impl GlxContext {
   pub fn new(x11_display: &x11::Display, screen_id: &x11::ScreenID) -> Result<Self, RuntimeError> {
      let display = glx::Display{
         ptr: x11_display.ptr,
      };

      let version = try!(glx::initialize(&display));

      println!("GLX Version {}.{}", version.major, version.minor);

      let config = try!(glx::choose_config(&display, screen_id));

      let visual = try!(glx::get_visual(&display, &config));

      println!("Visual bits per RGB {}", visual.bits_per_rgb);

      Ok(GlxContext {
         version: version,
         config: config,
      })
   }
}

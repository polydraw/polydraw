use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::glx;

pub struct GlxContext {
   pub version: glx::Version,
   pub config: glx::Config,
   pub context: glx::Context,
}

impl GlxContext {
   pub fn new(x11_display: &x11::Display, screen_id: &x11::ScreenID, window: &xcb::Window) -> Result<Self, RuntimeError> {
      let display = glx::Display{
         ptr: x11_display.ptr,
      };

      let version = try!(glx::initialize(&display));

      println!("GLX Version {}.{}", version.major, version.minor);

      let config = try!(glx::choose_config(&display, screen_id));

      let visual = try!(glx::get_visual(&display, &config));

      println!("Visual bits per RGB {}", visual.bits_per_rgb);

      let context = try!(glx::create_new_context(&display, &config));

      let rendering_area = try!(glx::create_rendering_area(&display, &config, window.window_id.id));

      println!("Rendering area {}", rendering_area);

      try!(glx::make_current(&display, rendering_area, &context));

      Ok(GlxContext {
         version: version,
         config: config,
         context: context,
      })
   }
}

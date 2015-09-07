use error::RuntimeError;

use sys::win32;
use sys::wgl;
use sys::gl;

pub struct WglContext {
   pub context: wgl::Context,
}

impl WglContext {
   pub fn new(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      try!(wgl::init_pixel_format(device_context));

      let context = try!(wgl::Context::create(device_context));

      Self::init_gl();

      Ok(WglContext {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl() {
      gl::load(wgl::Loader::new());
      gl::reset_pixelstore_alignment();
   }
}

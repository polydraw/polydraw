use error::{RuntimeError, VoidResult};
use frame::GPUFrame;

use sys::win32;
use sys::wgl;
use sys::gl;

pub struct WglContext {
   pub context: wgl::Context,
}

impl WglContext {
   pub fn new(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      let library = try!(win32::Library::new("opengl32.dll"));

      wgl::initialize(&library);

      try!(wgl::init_pixel_format(device_context));

      let context = try!(wgl::Context::create(device_context));

      try!(Self::init_gl(library));

      try!(wgl::swap_interval(0));

      Ok(WglContext {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl(library: win32::Library) -> VoidResult {
      let loader = wgl::Loader::new(Box::new(library));

      wgl::load_extra_functions(&loader);

      try!(gl::initialize(&loader));

      Ok(())
   }

   #[inline]
   pub fn create_gpu_frame(
      &self, width: u32, height: u32
   ) -> Result<Box<GPUFrame>, RuntimeError> {
      gl::frame::create_gpu_frame(width, height)
   }
}

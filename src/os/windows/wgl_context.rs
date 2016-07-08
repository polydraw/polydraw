use error::RuntimeError;
use frame::GPUFrame;

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

      try!(Self::init_gl());

      try!(wgl::swap_interval(0));

      Ok(WglContext {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl() -> Result<(), RuntimeError> {
      let loader = wgl::Loader::new();
      gl::load(&loader);
      wgl::load(&loader);

      try!(gl::reset_pixelstore_alignment());
      try!(gl::enable_framebuffer_srgb());

      Ok(())
   }

   #[inline]
   pub fn create_gpu_frame(
      &self, width: u32, height: u32
   ) -> Result<Box<GPUFrame>, RuntimeError> {

      match gl::frame::buffer::BufferFrame::new(width, height) {
         Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
         Err(e) => Err(e)
      }
   }
}

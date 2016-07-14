pub mod egl;
pub mod glx;

use error::{RuntimeError, VoidResult};
use frame::GPUFrame;

use sys::x11;
use sys::xcb;

use self::egl::EglContext;
use self::glx::GlxContext;

pub trait Context {
   fn new(
      x11_display: &x11::Display,
      screen_id: &x11::ScreenID,
      window: &xcb::Window
   ) -> Result<Self, RuntimeError> where Self: Sized;

   fn swap_buffers(&self) -> VoidResult;

   fn create_gpu_frame(&self, width: u32, height: u32) -> Result<Box<GPUFrame>, RuntimeError>;
}

pub fn create_context(
   x11_display: &x11::Display,
   screen_id: &x11::ScreenID,
   window: &xcb::Window
) -> Result<Box<Context>, RuntimeError> {

   match EglContext::new(x11_display, screen_id, window) {
      Ok(context) => Ok(Box::new(context)),

      Err(_) => {
         match GlxContext::new(x11_display, screen_id, window) {
            Ok(context) => Ok(Box::new(context)),
            Err(e) => Err(e)
         }
      }
   }
}

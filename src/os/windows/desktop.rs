use error::RuntimeError;

use sys::win32;

pub struct WindowsDesktop;

impl WindowsDesktop {
   pub fn new() -> Result<Self, RuntimeError> {
      Ok(WindowsDesktop)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      win32::DeviceMode::enumerate().screen_size()
   }
}

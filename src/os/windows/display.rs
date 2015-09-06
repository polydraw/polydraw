use error::RuntimeError;

use sys::win32;

pub struct WindowsDisplay;

impl WindowsDisplay {
   pub fn new() -> Result<Self, RuntimeError> {
      Ok(WindowsDisplay)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      win32::DeviceMode::enumerate().screen_size()
   }
}

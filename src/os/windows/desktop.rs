use error::RuntimeError;

pub struct WindowsDesktop;

impl WindowsDesktop {
   pub fn new() -> Result<Self, RuntimeError> {
      Ok(WindowsDesktop)
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      (1920, 1080)
   }
}

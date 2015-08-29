use error::RuntimeError;

pub struct LinuxWindow;

impl LinuxWindow {
   #[allow(unused_variables)]
   pub fn new(title: &str) -> Result<Self, RuntimeError> {
      Ok(LinuxWindow)
   }
}

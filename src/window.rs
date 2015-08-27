#[cfg(target_os = "linux")]
pub use os::linux::window::LinuxWindow as Window;

pub struct WindowCreator {
   title: String,
   width: Option<usize>,
   height: Option<usize>,
   centered: bool,
}

impl WindowCreator {
   pub fn new(title: &str) -> Self {
      WindowCreator {
         title: String::from(title),
         width: None,
         height: None,
         centered: false,
      }
   }

   pub fn create(self) {

   }

   pub fn centered(mut self) -> Self {
      self.centered = true;
      self
   }

   pub fn size(mut self, width: usize, height: usize) -> Self {
      self.width = Some(width);
      self.height = Some(height);
      self
   }
}

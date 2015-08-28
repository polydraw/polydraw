#[cfg(target_os = "linux")]
pub use os::linux::window::LinuxWindow as OSWindow;

use application::Application;

pub struct Window {
   os_window: OSWindow,
}

impl Window {
   pub fn new(creator: WindowCreator) -> Self {
      Window {
         os_window: OSWindow::new(creator.title).unwrap()
      }
   }
}

pub struct WindowCreator<'a> {
   app: &'a mut Application,
   title: &'a str,
   width: Option<usize>,
   height: Option<usize>,
   centered: bool,
}

impl<'a> WindowCreator<'a> {
   pub fn new(app: &'a mut Application, title: &'a str) -> Self {
      WindowCreator {
         app: app,
         title: title,
         width: None,
         height: None,
         centered: false,
      }
   }

   pub fn create(self) -> Window {
      Window::new(self)
   }

   pub fn centered(mut self, centered: bool) -> Self {
      self.centered = centered;
      self
   }

   pub fn size(mut self, width: usize, height: usize) -> Self {
      self.width = Some(width);
      self.height = Some(height);
      self
   }
}

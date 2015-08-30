use error::RuntimeError;

#[cfg(target_os = "linux")]
pub use os::linux::window::LinuxWindow as OSWindow;

use application::Application;

pub struct Window {
   pub os_window: OSWindow,
}

impl Window {
   pub fn new(os_window: OSWindow) -> Self {
      Window {
         os_window: os_window
      }
   }
}

pub struct WindowCreator<'a> {
   app: &'a mut Application,
   title: &'a str,
   x: Option<u32>,
   y: Option<u32>,
   width: Option<u32>,
   height: Option<u32>,
}

impl<'a> WindowCreator<'a> {
   pub fn new(app: &'a mut Application, title: &'a str) -> Self {
      WindowCreator {
         app: app,
         title: title,
         width: None,
         height: None,
         x: None,
         y: None,
      }
   }

   pub fn create(self) -> Result<Window, RuntimeError> {
      let (screen_width, screen_height) = self.app.screen_size();

      let width = match self.width {
         Some(width) => width,
         None => 3 * screen_width / 4
      };

      let height = match self.height {
         Some(height) => height,
         None => 3 * screen_height / 4
      };

      let x = match self.x {
         Some(x) => x,
         None => (screen_width - width) / 2
      };

      let y = match self.y {
         Some(y) => y,
         None => (screen_height - height) / 2
      };

      self.app.create_window(self.title, x, y, width, height)
   }

   pub fn size(mut self, width: u32, height: u32) -> Self {
      self.width = Some(width);
      self.height = Some(height);
      self
   }

   pub fn position(mut self, x: u32, y: u32) -> Self {
      self.x = Some(x);
      self.y = Some(y);
      self
   }
}

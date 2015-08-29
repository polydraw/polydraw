#[cfg(target_os = "linux")]
pub use os::linux::window::LinuxWindow as OSWindow;

use application::Application;

pub struct Window {
   os_window: OSWindow,
}

impl Window {
   pub fn new(title: &str) -> Self {
      Window {
         os_window: OSWindow::new(title).unwrap()
      }
   }
}

pub struct WindowCreator<'a> {
   app: &'a mut Application,
   title: &'a str,
   x: u32,
   y: u32,
   width: u32,
   height: u32,
}

impl<'a> WindowCreator<'a> {
   pub fn new(app: &'a mut Application, title: &'a str) -> Self {
      let (desktop_width, desktop_height) = app.desktop_size();

      let width = 3 * desktop_width / 4;
      let height = 3 * desktop_height / 4;
      let x = (desktop_width - width) / 2;
      let y = (desktop_height - height) / 2;

      WindowCreator {
         app: app,
         title: title,
         width: width,
         height: height,
         x: x,
         y: y,
      }
   }

   pub fn create(self) -> Window {
      Window::new(self.title)
   }

   pub fn size(mut self, width: u32, height: u32) -> Self {
      self.width = width;
      self.height = height;
      self
   }

   pub fn position(mut self, x: u32, y: u32) -> Self {
      self.x = x;
      self.y = y;
      self
   }
}

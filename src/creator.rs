pub use application::{OsDesktop, Application};

pub struct ApplicationCreator<'a> {
   desktop: OsDesktop,
   title: Option<&'a str>,
   x: Option<u32>,
   y: Option<u32>,
   width: Option<u32>,
   height: Option<u32>,
}

impl<'a> ApplicationCreator<'a> {
   pub fn new(desktop: OsDesktop) -> Self {
      ApplicationCreator {
         desktop: desktop,
         title: None,
         x: None,
         y: None,
         width: None,
         height: None,
      }
   }

   pub fn title(mut self, title: &'a str) -> Self {
      self.title = Some(title);
      self
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

   pub fn run(self) {
      let title = match self.title {
         Some(title) => title,
         None => "PolyDraw"
      };

      let (screen_width, screen_height) = self.desktop.screen_size();

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

      Application::create(self.desktop, title, x, y, width, height).run()
   }
}

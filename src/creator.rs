pub use super::application::{OsDisplay, Application};
pub use super::renderer::{Renderer, NullRenderer};

pub struct ApplicationCreator<'a> {
   display: OsDisplay,
   renderer: Option<&'a mut Renderer>,
   title: Option<&'a str>,
   x: Option<u32>,
   y: Option<u32>,
   width: Option<u32>,
   height: Option<u32>,
}

impl<'a> ApplicationCreator<'a> {
   pub fn new(display: OsDisplay) -> Self {
      ApplicationCreator {
         display: display,
         renderer: None,
         title: None,
         x: None,
         y: None,
         width: None,
         height: None,
      }
   }

   pub fn renderer(mut self, renderer: &'a mut Renderer) -> Self {
      self.renderer = Some(renderer);
      self
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
      let mut null_renderer = NullRenderer;

      let renderer: &mut Renderer = match self.renderer {
         Some(mut renderer) => renderer,
         None => &mut null_renderer as &mut Renderer
      };

      let title = match self.title {
         Some(title) => title,
         None => "PolyDraw"
      };

      let (screen_width, screen_height) = self.display.screen_size();

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

      Application::create(self.display, title, x, y, width, height).run(renderer)
   }
}

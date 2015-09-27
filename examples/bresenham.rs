extern crate polydraw;

use polydraw::{Application, Renderer, RenderFrame};
use polydraw::draw::{RGB, bresenham};

struct BresenhamRenderer;

impl BresenhamRenderer {
   fn new() -> Self {
      BresenhamRenderer
   }
}

impl Renderer for BresenhamRenderer {
   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let color = RGB::new(255, 191, 127);

      let x2 = frame.width as i32 - 1;
      let y2 = frame.height as i32 - 1;
      bresenham(frame, 0, 0, x2, y2, &color);

      let x1 = frame.width as i32 - 1;
      let y2 = frame.height as i32 - 1;
      bresenham(frame, x1, 0, 0, y2, &color);
   }
}

fn main() {
   let mut renderer = BresenhamRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Bresenham")
      .run();
}

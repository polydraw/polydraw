extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::{RGB, bresenham, hline, vline};

struct LinesRenderer;

impl LinesRenderer {
   fn new() -> Self {
      LinesRenderer
   }
}

impl Renderer for LinesRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let color = RGB::new(127, 223, 255);

      let center_x = frame.width as i32 / 2;
      let center_y = frame.height as i32 / 2;

      let top_y = center_y + 100;
      let bottom_y = center_y - 100;
      let right_x = center_x + 250;
      let left_x = center_x - 250;

      let end_x = frame.width as i32 - 1;
      let end_y = frame.height as i32 - 1;

      bresenham(frame, 0, 0, left_x, bottom_y, &color);
      bresenham(frame, 0, end_y, left_x, top_y, &color);
      bresenham(frame, end_x, 0, right_x, bottom_y, &color);
      bresenham(frame, end_x, end_y, right_x, top_y, &color);

      hline(frame, left_x, right_x, top_y, &color);
      hline(frame, left_x, right_x, bottom_y, &color);
      vline(frame, left_x, bottom_y, top_y, &color);
      vline(frame, right_x, bottom_y, top_y, &color);
   }
}

fn main() {
   let mut renderer = LinesRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Lines")
      .run();
}

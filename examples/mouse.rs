extern crate polydraw;

use std::cmp::{min, max};

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;

struct MouseRenderer {
   mouse_x: i32,
   mouse_y: i32,
}

impl MouseRenderer {
   fn new() -> Self {
      MouseRenderer {
         mouse_x: 1000000,
         mouse_y: 1000000,
      }
   }
}

impl Renderer for MouseRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let color = RGB::new(33, 168, 222);

      let half = 35_u32;

      let x_start = max(0, self.mouse_x - half as i32);
      let x_end = min(frame.width as i32, self.mouse_x + half as i32);

      let y_start = max(0, self.mouse_y - half as i32);
      let y_end = min(frame.height as i32, self.mouse_y + half as i32);

      for y in y_start..y_end {
         for x in x_start..x_end {
            frame.put_pixel(x, y, &color);
         }
      }
   }

   fn mouse_moved(&mut self, x: i32, y: i32) {
      self.mouse_x = x;
      self.mouse_y = y;
   }
}

fn main() {
   let mut renderer = MouseRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Mouse")
      .run();
}

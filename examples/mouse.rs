extern crate polydraw;

use std::cmp::{min, max};

use polydraw::{Application, Renderer, RenderFrame};

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
   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let half = 35_u32;

      let x_start = max(0, self.mouse_x - half as i32);
      let x_end = min(frame.width as i32, self.mouse_x + half as i32);

      let y_start = max(0, self.mouse_y - half as i32);
      let y_end = min(frame.height as i32, self.mouse_y + half as i32);

      for y in y_start..y_end {
         let row_i = 3 * y * frame.width as i32;
         for x in x_start..x_end {
            let i = (3 * x + row_i) as usize;
            frame.data[i] = 33;
            frame.data[i + 1] = 168;
            frame.data[i + 2] = 222;
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

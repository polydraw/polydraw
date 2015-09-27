extern crate polydraw;

use std::cmp::{min, max};

use polydraw::{Application, Renderer, RenderFrame};

struct MouseRenderer {
   mouse_x: u32,
   mouse_y: u32,
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
      for i in 0 as usize..(frame.width * frame.height * 3) as usize {
         frame.data[i] = 0;
      }

      let half = 35_u32;

      let x_start = max(0, (self.mouse_x as i32) - half as i32) as u32;
      let x_end = min(frame.width, self.mouse_x + half);

      let y_start = max(0, (self.mouse_y as i32) - half as i32) as u32;
      let y_end = min(frame.height, self.mouse_y + half);

      for y in y_start..y_end {
         let row_i = 3 * y * frame.width;
         for x in x_start..x_end {
            let i = (3 * x + row_i) as usize;
            frame.data[i] = 33;
            frame.data[i + 1] = 168;
            frame.data[i + 2] = 222;
         }
      }
   }

   fn mouse_moved(&mut self, x: u32, y: u32) {
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

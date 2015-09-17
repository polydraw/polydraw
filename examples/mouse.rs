extern crate polydraw;

use std::cmp::{min, max};

use polydraw::{Application, Renderer, RenderFrame};

struct MouseRenderer {
   mouse_x: u32,
   mouse_y: u32,
   old: Option<(u32, u32)>,
}

impl MouseRenderer {
   fn new() -> Self {
      MouseRenderer {
         mouse_x: 0,
         mouse_y: 0,
         old: None,
      }
   }
}

impl Renderer for MouseRenderer {
   fn render(&mut self, frame: &mut RenderFrame) {
      let half = 35_u32;

      match self.old {
         Some((old_x, old_y)) => {
            let old_x_start = max(0, (old_x as i32) - half as i32) as u32;
            let old_x_end = min(frame.width, old_x + half);

            let old_y_start = max(0, (old_y as i32) - half as i32) as u32;
            let old_y_end = min(frame.height, old_y + half);

            for y in old_y_start..old_y_end {
               let row_i = 3 * y * frame.width;

               let mut i = (3 * old_x_start + row_i) as usize;
               frame.data[i] = 33;
               frame.data[i + 1] = 168;
               frame.data[i + 2] = 222;

               i = (3 * old_x_end + row_i) as usize;
               frame.data[i] = 33;
               frame.data[i + 1] = 168;
               frame.data[i + 2] = 222;
            }

            for x in old_x_start..old_x_end {
               let mut row_i = 3 * old_y_start * frame.width;

               let mut i = (3 * x + row_i) as usize;
               frame.data[i] = 33;
               frame.data[i + 1] = 168;
               frame.data[i + 2] = 222;

               row_i = 3 * old_y_end * frame.width;

               i = (3 * x + row_i) as usize;
               frame.data[i] = 33;
               frame.data[i + 1] = 168;
               frame.data[i + 2] = 222;
            }
         },
         None => {},
      }

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

      for y in y_start..y_end {
         let row_i = 3 * y * frame.width;

         let mut i = (3 * x_start + row_i) as usize;
         frame.data[i] = 255;
         frame.data[i + 1] = 255;
         frame.data[i + 2] = 255;

         i = (3 * x_end + row_i) as usize;
         frame.data[i] = 255;
         frame.data[i + 1] = 255;
         frame.data[i + 2] = 255;
      }

      for x in x_start..x_end {
         let mut row_i = 3 * y_start * frame.width;

         let mut i = (3 * x + row_i) as usize;
         frame.data[i] = 255;
         frame.data[i + 1] = 255;
         frame.data[i + 2] = 255;

         row_i = 3 * y_end * frame.width;

         i = (3 * x + row_i) as usize;
         frame.data[i] = 255;
         frame.data[i + 1] = 255;
         frame.data[i + 2] = 255;
      }

      self.old = Some((self.mouse_x, self.mouse_y));
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

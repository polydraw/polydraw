extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::geom::number::NumberOps;

const DIV_PER_PIXEL: i64 = 1000;

#[inline]
fn to_px(v: i64) -> i64 {
   v / DIV_PER_PIXEL
}

#[inline]
fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

struct CircleRenderer {
   center_x: i64,
   center_y: i64,
   radius: i64,
}

impl CircleRenderer {
   fn new() -> Self {
      let r = 300;
      CircleRenderer {
         center_x: from_px(r+100),
         center_y: from_px(r+100),
         radius: from_px(r),
      }
   }
}

impl Renderer for CircleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let color = RGB::new(127, 223, 255);

      let center_x = to_px(self.center_x);
      let center_y = to_px(self.center_y);

      for y in 0..to_px(self.radius) + 1 {
         let y_world = from_px(y);
         let x_world = on_circle(self.radius, y_world);
         let x = to_px(x_world);

         let real_y = center_y + y;
         let bottom_y = center_y - y;

         for i in -x..x+1 {
            let real_x = center_x + i;
            frame.put_pixel(
               real_x as i32,
               real_y as i32,
               &color
            );
            frame.put_pixel(
               real_x as i32,
               bottom_y as i32,
               &color
            );
         }
      }
   }
}

fn on_circle(radius: i64, coord: i64) -> i64 {
   (radius.pow(2) - coord.pow(2)).isqrt()
}

fn main() {
   let mut renderer = CircleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Circle")
      .run();
}

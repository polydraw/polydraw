extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;

const DIV_PER_PIXEL: i64 = 1000;
const DOUBLE_PIXEL_AREA: i64 = DIV_PER_PIXEL * DIV_PER_PIXEL * 2;

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

      frame.put_pixel(to_px(self.center_x) as i32, to_px(self.center_y) as i32, &color);
   }
}

fn main() {
   let mut renderer = CircleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Circle")
      .run();
}

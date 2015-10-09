extern crate polydraw;

use std::cmp::{min, max};

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::geom::point::Point;
use polydraw::geom::triangle::Triangle;

struct TriangleRenderer {
   tr: Triangle,
}

impl TriangleRenderer {
   fn new() -> Self {
      let mult = 50;
      let tr = Triangle::new(
         Point::new(2 * mult, 9 * mult),
         Point::new(13 * mult, 12 * mult),
         Point::new(7 * mult, 2 * mult),
      );

      println!("orientation {:?}", orientation(&tr.a, &tr.b, &tr.c));

      TriangleRenderer {
         tr: tr
      }
   }
}

impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let color = RGB::new(127, 223, 255);

      self.tr.a.x += 1;
      if self.tr.a.x >= frame.width as i64 {
         self.tr.a.x = 0;
      }

      let a = self.tr.a;
      let b = self.tr.b;
      let c = self.tr.c;

      let min_x = max(min3(a.x, b.x, c.x), 0);
      let min_y = max(min3(a.y, b.y, c.y), 0);
      let max_x = min(max3(a.x, b.x, c.x), frame.width as i64 - 1);
      let max_y = min(max3(a.y, b.y, c.y), frame.height as i64 - 1);

      for y in min_y..max_y+1 {
         for x in min_x..max_x+1 {
            let p = Point::new(x, y);
            let w0 = orientation(&b, &c, &p);
            let w1 = orientation(&c, &a, &p);
            let w2 = orientation(&a, &b, &p);

            if w0.signum() == w1.signum() && w0.signum() == w2.signum() {
               frame.put_pixel(x as i32, y as i32, &color);
            }
         }
      }
   }
}

#[inline]
pub fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
   min(min(v1, v2), v3)
}

#[inline]
pub fn max3<T: Ord>(v1: T, v2: T, v3: T) -> T {
   max(max(v1, v2), v3)
}

#[inline]
fn orientation(a: &Point, b: &Point, c: &Point) -> i64 {
   (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle")
      .run();
}

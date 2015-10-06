extern crate polydraw;

use polydraw::{Application, Renderer, RenderFrame};
use polydraw::draw::{RGB, bresenham};
use polydraw::geom::point::Point;
use polydraw::geom::triangle::Triangle;

struct TriangleRenderer {
   triangle: Triangle,
}

impl TriangleRenderer {
   fn new() -> Self {
      let mult = 50;
      let triangle = Triangle::new(
         Point::new(2 * mult, 9 * mult),
         Point::new(13 * mult, 12 * mult),
         Point::new(7 * mult, 2 * mult),
      );

      TriangleRenderer {
         triangle: triangle
      }
   }
}

impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let color = RGB::new(127, 223, 255);

      bresenham(frame, self.triangle.v0.x as i32, self.triangle.v0.y as i32, self.triangle.v1.x as i32, self.triangle.v1.y as i32, &color);
      bresenham(frame, self.triangle.v1.x as i32, self.triangle.v1.y as i32, self.triangle.v2.x as i32, self.triangle.v2.y as i32, &color);
      bresenham(frame, self.triangle.v2.x as i32, self.triangle.v2.y as i32, self.triangle.v0.x as i32, self.triangle.v0.y as i32, &color);
   }
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle")
      .run();
}

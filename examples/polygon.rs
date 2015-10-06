extern crate polydraw;

use std::cmp::Ordering;

use polydraw::{Application, Renderer, RenderFrame};
use polydraw::draw::{RGB, bresenham};
use polydraw::geom::point::Point;
use polydraw::geom::polygon::Polygon;

struct PolygonRenderer {
   polygon: Polygon,
}

impl PolygonRenderer {
   fn new() -> Self {
      let mult = 50;
      let mut polygon = Polygon::new(vec![
         Point::new(2 * mult, 9 * mult),
         Point::new(13 * mult, 12 * mult),
         Point::new(9 * mult, 3 * mult),
         Point::new(7 * mult, 2 * mult),
      ]);

      PolygonRenderer {
         polygon: polygon
      }
   }

   fn printall(&self) {
      let mut prev = self.polygon.points[self.polygon.points.len() - 1];

      for point in &self.polygon.points {
         if cmp_points(&prev, &point) == Ordering::Less {
            println!("{} {} - {} {}", prev.x, prev.y, point.x, point.y);
         } else {
            println!("{} {} - {} {}", point.x, point.y, prev.x, prev.y);
         }
         prev = point.clone();
      }
   }
}

impl Renderer for PolygonRenderer {
   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let color = RGB::new(127, 223, 255);

      let mut prev = self.polygon.points[self.polygon.points.len() - 1];

      for point in &self.polygon.points {
         bresenham(frame, prev.x as i32, prev.y as i32, point.x as i32, point.y as i32, &color);
         prev = point.clone();
      }
   }
}

fn cmp_points(pt1: &Point, pt2: &Point) -> Ordering {
   if pt1.y < pt2.y {
      Ordering::Less
   } else if pt1.y > pt2.y {
      Ordering::Greater
   } else if pt1.x < pt2.x {
      Ordering::Less
   } else if pt1.x > pt2.x {
      Ordering::Greater
   } else {
      Ordering::Equal
   }
}

fn main() {
   let mut renderer = PolygonRenderer::new();

   renderer.printall();

   Application::new()
      .renderer(&mut renderer)
      .title("Polygon")
      .run();
}

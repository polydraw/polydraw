extern crate polydraw;

use polydraw::Application;
use polydraw::devel::{Scene, Poly, DevelRenderer, SUBDIVISIONS};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;

fn multiply_points(mut points: Vec<Point>) -> Vec<Point> {
   for point in &mut points {
      point.x *= SUBDIVISIONS as i64;
      point.y *= SUBDIVISIONS as i64;
   }

   points
}

fn main() {
   let mut scene = Scene::new();

   let poly_a = Poly::new(
      multiply_points(vec![
         Point::new(1100, 400),
         Point::new(200, 200),
         Point::new(500, 700),
      ]),
      RGB::new(34, 78, 29),
   );

   let poly_b = Poly::new(
      multiply_points(vec![
         Point::new(100, 500),
         Point::new(900, 700),
         Point::new(900, 200),
         Point::new(700, 100),
      ]),
      RGB::new(128, 59, 89),
   );

   scene.push(poly_a);
   scene.push(poly_b);

   let mut renderer = DevelRenderer::new(scene);

   Application::new()
      .renderer(&mut renderer)
      .title("Development Rasterizer")
      .size(1200, 800)
      .run();
}

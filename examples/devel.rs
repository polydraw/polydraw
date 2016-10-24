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
      vec![
         multiply_points(vec![
            Point::new(1100, 400),
            Point::new(200, 200),
            Point::new(500, 700),
         ]),
      ],
      RGB::new(34, 78, 29),
   );

   let poly_b = Poly::new(
      vec![
         multiply_points(vec![
            Point::new(100, 500),
            Point::new(900, 700),
            Point::new(900, 200),
            Point::new(700, 100),
         ]),
      ],
      RGB::new(128, 59, 89),
   );

   let poly_c = Poly::new(
      vec![
         multiply_points(vec![
            Point::new(270, 120),
            Point::new(120, 450),
            Point::new(510, 690),
            Point::new(570, 240),
         ]),
         multiply_points(vec![
            Point::new(300, 180),
            Point::new(420, 620),
            Point::new(480, 420),
         ]),
      ],
      RGB::new(215, 12, 96),
   );

   scene.push(Box::new(poly_a));
   scene.push(Box::new(poly_b));
   scene.push(Box::new(poly_c));

   let mut renderer = DevelRenderer::new(scene);

   Application::new()
      .renderer(&mut renderer)
      .title("Development Rasterizer")
      .size(1200, 800)
      .run();
}

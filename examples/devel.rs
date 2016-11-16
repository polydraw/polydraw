extern crate polydraw;

use polydraw::Application;
use polydraw::devel::{Scene, Poly, DevelRenderer, SUBDIVISIONS};
use polydraw::data::IntPoint;
use polydraw::draw::RGB;

fn multiply_points(mut points: Vec<IntPoint>) -> Vec<IntPoint> {
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
            IntPoint::new(1100, 400),
            IntPoint::new(200, 200),
            IntPoint::new(500, 700),
         ]),
      ],
      RGB::new(34, 78, 29),
   );

   let poly_b = Poly::new(
      vec![
         multiply_points(vec![
            IntPoint::new(100, 500),
            IntPoint::new(900, 700),
            IntPoint::new(900, 200),
            IntPoint::new(700, 100),
         ]),
      ],
      RGB::new(128, 59, 89),
   );

   let poly_c = Poly::new(
      vec![
         multiply_points(vec![
            IntPoint::new(270, 120),
            IntPoint::new(120, 450),
            IntPoint::new(510, 690),
            IntPoint::new(570, 240),
         ]),
         multiply_points(vec![
            IntPoint::new(300, 180),
            IntPoint::new(420, 620),
            IntPoint::new(480, 420),
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

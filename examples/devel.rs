extern crate polydraw;

use polydraw::Application;
use polydraw::devel::{Scene, Poly, DevelRenderer};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


fn main() {
   let mut scene = Scene::new();

   let poly_a = Poly::new(
      vec![
         Point::new(1100, 400),
         Point::new(200, 200),
         Point::new(500, 700),
      ],
      RGB::new(34, 78, 29),
   );

   let poly_b = Poly::new(
      vec![
         Point::new(100, 500),
         Point::new(900, 700),
         Point::new(900, 200),
         Point::new(700, 100),
      ],
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

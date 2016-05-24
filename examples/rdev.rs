extern crate polydraw;

use polydraw::geom::point::Point;
use polydraw::draw::RGB;


struct Poly {
   points: Vec<Point>,
   color: RGB,
}

struct Scene {
   background: RGB,
   polys: Vec<Poly>,
}

impl Scene {
   #[inline]
   fn new(background: RGB) -> Self {
      Scene {
         background: background,
         polys: Vec::new(),
      }
   }

   #[inline]
   pub fn push(&mut self, poly: Poly) {
      self.polys.push(poly);
   }
}


fn main() {
   let mut scene = Scene::new(RGB::new(0, 0, 0));

   let poly_a = Poly {
      points: vec![Point::new(10, 10), Point::new(30, 20), Point::new(20, 30)],
      color: RGB::new(34, 78, 29),
   };

   let poly_b = Poly {
      points: vec![Point::new(40, 40), Point::new(5, 20), Point::new(50, 0)],
      color: RGB::new(128, 59, 89),
   };

   scene.push(poly_a);
   scene.push(poly_b);

   println!("{:?}", scene.background);

   for poly in scene.polys {
      println!("{:?} - {:?}", poly.points, poly.color);
   }
}


extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


struct Poly {
   points: Vec<Point>,
   color: RGB,
}

struct Scene {
   polys: Vec<Poly>,
}

impl Scene {
   #[inline]
   fn new() -> Self {
      Scene {
         polys: Vec::new(),
      }
   }

   #[inline]
   pub fn push(&mut self, poly: Poly) {
      self.polys.push(poly);
   }
}


struct DevRenderer {
   scene: Scene,
}


impl DevRenderer {
   fn new(scene: Scene) -> Self {
      DevRenderer {
         scene: scene,
      }
   }
}



impl Renderer for DevRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let white = RGB::new(255, 255, 255);

      for poly in self.scene.polys.iter() {
         for point in poly.points.iter() {
            frame.put_pixel(point.x as i32, point.y as i32, &white)
         }
      }
   }
}



fn main() {
   let mut scene = Scene::new();

   let poly_a = Poly {
      points: vec![
         Point::new(200, 200),
         Point::new(1100, 400),
         Point::new(500, 700),
      ],
      color: RGB::new(34, 78, 29),
   };

   let poly_b = Poly {
      points: vec![
         Point::new(900, 200),
         Point::new(900, 1000),
         Point::new(100, 500),
         Point::new(700, 100),
      ],
      color: RGB::new(128, 59, 89),
   };

   scene.push(poly_a);
   scene.push(poly_b);

   let mut renderer = DevRenderer::new(scene);

   Application::new()
      .renderer(&mut renderer)
      .title("Dev Rasterizer")
      .run();
}


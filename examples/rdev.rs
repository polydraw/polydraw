extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


struct Poly {
   points: Vec<Point>,
   color: RGB,
}


impl Poly {
   fn min_y_index(&self) -> usize {
      let mut max_after = false;

      let (first, rest) = self.points.split_first().unwrap();

      let mut min_y = first.y;
      let mut min_index: usize = 0;

      for (index, point) in rest.iter().enumerate() {
         if point.y > min_y {
            if max_after {
               return min_index;
            }
         } else {
            max_after = true;
            min_index = index + 1;
            min_y = point.y;
         }
      }

      min_index
   }

   fn max_y_index(&self) -> usize {
      let mut min_after = false;

      let (first, rest) = self.points.split_first().unwrap();

      let mut max_y = first.y;
      let mut max_index: usize = 0;

      for (index, point) in rest.iter().enumerate() {
         if point.y < max_y {
            if min_after {
               return max_index;
            }
         } else {
            min_after = true;
            max_index = index + 1;
            max_y = point.y;
         }
      }

      max_index
   }
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

   fn _render_poly(&self, frame: &mut Frame, poly: &Poly) {
      let min_y_index = poly.min_y_index();
      let max_y_index = poly.max_y_index();

      println!("MIN [{:?}] = {:?}", min_y_index, poly.points[min_y_index]);

      println!("MAX [{:?}] = {:?}", max_y_index, poly.points[max_y_index]);

      for point in poly.points.iter() {
         frame.put_pixel(point.x as i32, point.y as i32, &poly.color)
      }
   }
}


impl Renderer for DevRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      for poly in self.scene.polys.iter() {
         self._render_poly(frame, poly);
      }

      panic!("Bye!");
   }
}


fn main() {
   let mut scene = Scene::new();

   let poly_a = Poly {
      points: vec![
         Point::new(1100, 400),
         Point::new(500, 700),
         Point::new(200, 200),
      ],
      color: RGB::new(34, 78, 29),
   };

   let poly_b = Poly {
      points: vec![
         Point::new(100, 500),
         Point::new(700, 100),
         Point::new(900, 200),
         Point::new(900, 1000),
      ],
      color: RGB::new(128, 59, 89),
   };

   scene.push(poly_a);
   scene.push(poly_b);

   let mut renderer = DevRenderer::new(scene);

   Application::new()
      .renderer(&mut renderer)
      .title("Dev Rasterizer")
      .size(800, 450)
      .run();
}


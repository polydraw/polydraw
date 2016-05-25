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

   fn _render_poly(&self, frame: &mut Frame, poly: &Poly) {
      let min_y_index = _min_y_index(poly);

      println!("[{:?}] = {:?}", min_y_index, poly.points[min_y_index]);

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


fn _min_y_index(poly: &Poly) -> usize {
   let mut max_after = false;

   let (first, rest) = poly.points.split_first().unwrap();

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


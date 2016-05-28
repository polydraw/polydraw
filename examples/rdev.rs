extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


struct Poly {
   points: Vec<Point>,
   color: RGB,
}


impl Poly {
   fn left_right_points(&self) -> (Vec<Point>, Vec<Point>) {
      let (min_y_i, max_y_i) = self._min_max_y_index();

      let (left_count, right_count) = self._left_right_count(min_y_i, max_y_i);

      return (
         self._left_points(min_y_i, left_count),
         self._right_points(min_y_i, right_count)
      )
   }

   fn _left_points(&self, min_y_i: usize, left_count: usize) -> Vec<Point> {
      let mut left = Vec::with_capacity(left_count);

      let points_len = self.points.len();

      let mut index = min_y_i;

      for _ in 0..left_count {
         left.push(self.points[index]);

         if index == 0 {
            index = points_len - 1;
         } else {
            index -= 1;
         }
      }

      return left;
   }

   fn _right_points(&self, min_y_i: usize, right_count: usize) -> Vec<Point> {
      let mut right = Vec::with_capacity(right_count);

      let points_len = self.points.len();

      let mut index = min_y_i;

      for _ in 0..right_count {
         if index == points_len {
            index = 0;
         }

         right.push(self.points[index]);

         index += 1;
      }

      return right;
   }

   fn _left_right_count(&self, min_y_i: usize, max_y_i: usize) -> (usize, usize) {
      let points_len = self.points.len();

      let left_count = if max_y_i > min_y_i {
         max_y_i - min_y_i + 1
      } else {
         points_len - min_y_i + max_y_i + 1
      };

      let right_count = points_len - left_count + 2;

      return (left_count, right_count)
   }

   fn _min_max_y_index(&self) -> (usize, usize) {
      let (first, rest) = self.points.split_first().unwrap();

      let mut min_y = first.y;
      let mut max_y = min_y;
      let mut min_index: usize = 0;
      let mut max_index: usize = 0;

      for (index, point) in rest.iter().enumerate() {
         if point.y < min_y {
            min_index = index + 1;
            min_y = point.y;
         } else if point.y > max_y {
            max_index = index + 1;
            max_y = point.y;
         }
      }

      (min_index, max_index)
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
      let (left_points, right_points) = poly.left_right_points();

      println!("LEFT");

      for (index, point) in left_points.iter().enumerate() {
         println!("[{}] = {:?}", index, point);
      }

      println!("RIGHT");

      for (index, point) in right_points.iter().enumerate() {
         println!("[{}] = {:?}", index, point);
      }

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


extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


struct Poly {
   points: Vec<Point>,
   color: RGB,
}


impl Poly {
   fn left_right_edges(&self) -> (i64, i64, Vec<Edge>, Vec<Edge>) {
      let (min_y, max_y, min_i, max_i) = self._min_max_y_index();

      let (left_count, right_count) = self._left_right_count(min_i, max_i);

      let left_points = self._left_points(min_i, left_count);

      let left_edges = self._edges_from_points(&left_points);

      let right_points = self._right_points(min_i, right_count);

      let right_edges = self._edges_from_points(&right_points);

      (min_y, max_y, left_edges, right_edges)
   }

   fn _edges_from_points(&self, points: &Vec<Point>) -> Vec<Edge> {
      let mut edges: Vec<Edge> = Vec::with_capacity(points.len() - 1);

      for index in 0..points.len() - 1 {
         edges.push(
            Edge::new(points[index], points[index + 1])
         );
      }

      edges
   }

   fn _left_points(&self, min_i: usize, left_count: usize) -> Vec<Point> {
      let mut left = Vec::with_capacity(left_count);

      let points_len = self.points.len();

      let mut index = min_i;

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

   fn _right_points(&self, min_i: usize, right_count: usize) -> Vec<Point> {
      let mut right = Vec::with_capacity(right_count);

      let points_len = self.points.len();

      let mut index = min_i;

      for _ in 0..right_count {
         if index == points_len {
            index = 0;
         }

         right.push(self.points[index]);

         index += 1;
      }

      return right;
   }

   fn _left_right_count(&self, min_i: usize, max_i: usize) -> (usize, usize) {
      let points_len = self.points.len();

      let right_count = if max_i > min_i {
         max_i - min_i + 1
      } else {
         points_len - min_i + max_i + 1
      };

      let left_count = points_len - right_count + 2;

      return (left_count, right_count)
   }

   fn _min_max_y_index(&self) -> (i64, i64, usize, usize) {
      let (first, rest) = self.points.split_first().unwrap();

      let mut min_y = first.y;
      let mut max_y = min_y;
      let mut min_i: usize = 0;
      let mut max_i: usize = 0;

      for (i, point) in rest.iter().enumerate() {
         if point.y < min_y {
            min_i = i + 1;
            min_y = point.y;
         } else if point.y > max_y {
            max_i = i + 1;
            max_y = point.y;
         }
      }

      (min_y, max_y, min_i, max_i)
   }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
   p1: Point,
   p2: Point,
}

impl Edge {
   #[inline]
   fn new(p1: Point, p2: Point) -> Self {
      Edge {
         p1: p1,
         p2: p2,
      }
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
      let (min_y, max_y, left_edges, right_edges) = poly.left_right_edges();

      println!("min_y {:?}, max_y {:?}", min_y, max_y);

      println!("LEFT");

      for (index, edge) in left_edges.iter().enumerate() {
         println!("[{}] = {:?}", index, edge);
      }

      println!("RIGHT");

      for (index, edge) in right_edges.iter().enumerate() {
         println!("[{}] = {:?}", index, edge);
      }

      let mut left_i = 0;
      let mut right_i = 0;

      let mut left_edge = left_edges[left_i];
      let mut right_edge = right_edges[right_i];

      for y in min_y..max_y + 1 {
         if left_edge.p2.y < y {
            left_i += 1;
            left_edge = left_edges[left_i];
         }

         if right_edge.p2.y < y {
            right_i += 1;
            right_edge = right_edges[right_i];
         }

         println!("Y {:?} LEFT {:?} ", y, left_edge);
         println!("Y {:?} RIGHT {:?} ", y, right_edge);
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


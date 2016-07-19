use geom::point::Point;
use renderer::Renderer;
use frame::Frame;

use super::Scene;

pub struct DevelRenderer {
   scene: Scene,
   width: u32,
   height: u32,
}


impl DevelRenderer {
   #[inline]
   pub fn new(scene: Scene) -> Self {
      DevelRenderer {
         scene: scene,
         width: 0,
         height: 0,
      }
   }
}


impl Renderer for DevelRenderer {
   fn init(&mut self, width: u32, height: u32) {
      self.width = width;
      self.height = height;
   }

   fn render(&mut self, _: &mut Frame) {
      let points = &self.scene.polys[0].points;

      for i in 0..points.len() {
         println!("{:?}", points[i]);
      }

      let (left, right) = left_right_edges(points);

      for edge in &left {
         println!("{:?}", edge);
      }

      for edge in &right {
         println!("{:?}", edge);
      }

      panic!("END");
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

fn left_right_edges(points: &Vec<Point>) -> (Vec<Edge>, Vec<Edge>){
   let (_, _, min_i, max_i) = min_max_y_index(points);

   let left = left_edges(points, min_i, max_i);
   let right = right_edges(points, min_i, max_i);

   (left, right)
}

fn left_edges(points: &Vec<Point>, min_i: usize, max_i: usize) -> Vec<Edge> {
   let mut edges = Vec::new();

   let mut curr_i = min_i;
   let mut prev_i = curr_i;

   loop {
      curr_i += 1;

      if curr_i == points.len() {
         curr_i = 0;
      }

      edges.push(Edge::new(points[prev_i], points[curr_i]));

      prev_i = curr_i;

      if curr_i == max_i {
         break;
      }
   }

   edges
}

fn right_edges(points: &Vec<Point>, min_i: usize, max_i: usize) -> Vec<Edge> {
   let mut edges = Vec::new();

   let mut curr_i = min_i;
   let mut prev_i = curr_i;

   loop {
      if curr_i == 0 {
         curr_i = points.len() - 1;
      } else {
         curr_i -= 1;
      }

      edges.push(Edge::new(points[prev_i], points[curr_i]));

      prev_i = curr_i;

      if curr_i == max_i {
         break;
      }
   }

   edges
}


fn min_max_y_index(points: &Vec<Point>) -> (i64, i64, usize, usize) {
   let (first, rest) = points.split_first().unwrap();

   let mut min_y = first.y;
   let mut max_y = min_y;
   let mut min_i = 0;
   let mut max_i = 0;

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

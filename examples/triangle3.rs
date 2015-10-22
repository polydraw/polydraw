#![allow(dead_code)]
extern crate polydraw;

use std::iter;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgeType {
   Inclined,
   InclinedRev,
   Horizontal,
   HorizontalRev,
   Vertical,
   VerticalRev,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
   etype: EdgeType,
   p1: usize,
   p2: usize,
}

impl Edge {
   #[inline]
   pub fn new(etype: EdgeType, p1: usize, p2: usize) -> Self {
      Edge {
         etype: etype,
         p1: p1,
         p2: p2,
      }
   }
}

impl Default for Edge {
   #[inline]
   fn default() -> Edge {
      Edge::new(EdgeType::Inclined, 0, 0)
   }
}

pub fn fill_default<T>(capacity: usize) -> Vec<T> where T: Default + Clone {
   iter::repeat(T::default()).take(capacity).collect()
}

struct TriangleRenderer {
   triangles: Vec<Edge>,
   original: Vec<Point>,

   above: Vec<Edge>,
   below: Vec<Edge>,
   points: Vec<Point>,
}

impl TriangleRenderer {
   fn new() -> Self {
      let triangles = vec![
         // A
         Edge::new(EdgeType::Vertical, 0, 1),
         Edge::new(EdgeType::InclinedRev, 1, 2),
         Edge::new(EdgeType::InclinedRev, 2, 0),

         // B
         Edge::new(EdgeType::Inclined, 0, 2),
         Edge::new(EdgeType::InclinedRev, 2, 3),
         Edge::new(EdgeType::InclinedRev, 3, 0),

         // C
         Edge::new(EdgeType::Inclined, 0, 3),
         Edge::new(EdgeType::InclinedRev, 3, 4),
         Edge::new(EdgeType::HorizontalRev, 4, 0),

         // D
         Edge::new(EdgeType::Inclined, 4, 3),
         Edge::new(EdgeType::Inclined, 3, 5),
         Edge::new(EdgeType::InclinedRev, 5, 4),

         // E
         Edge::new(EdgeType::Inclined, 4, 5),
         Edge::new(EdgeType::Inclined, 5, 6),
         Edge::new(EdgeType::VerticalRev, 6, 4),

         // F
         Edge::new(EdgeType::Inclined, 3, 2),
         Edge::new(EdgeType::Inclined, 2, 7),
         Edge::new(EdgeType::InclinedRev, 7, 3),

         // G
         Edge::new(EdgeType::Inclined, 3, 7),
         Edge::new(EdgeType::InclinedRev, 7, 5),
         Edge::new(EdgeType::InclinedRev, 5, 3),

         // H
         Edge::new(EdgeType::Inclined, 2, 1),
         Edge::new(EdgeType::InclinedRev, 1, 7),
         Edge::new(EdgeType::InclinedRev, 7, 2),

         // I
         Edge::new(EdgeType::Inclined, 5, 7),
         Edge::new(EdgeType::Inclined, 7, 6),
         Edge::new(EdgeType::InclinedRev, 6, 5),

         // J
         Edge::new(EdgeType::Inclined, 7, 1),
         Edge::new(EdgeType::Horizontal, 1, 6),
         Edge::new(EdgeType::InclinedRev, 6, 7),
      ];

      let original = vec![
         Point::new(0, 0),   // 0
         Point::new(0, 10),  // 1
         Point::new(2, 5),   // 2
         Point::new(3, 1),   // 3
         Point::new(10, 0),  // 4
         Point::new(8, 5),   // 5
         Point::new(10, 10), // 6
         Point::new(7, 9),   // 7
      ];

      TriangleRenderer {
         triangles: triangles,
         original: original,

         above: fill_default(262144),
         below: fill_default(262144),
         points: fill_default(262144),
      }
   }
}


impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();
   }
}


fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle3")
      .run();
}

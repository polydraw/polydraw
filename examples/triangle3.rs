#![allow(dead_code)]
extern crate polydraw;

use std::iter;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgeType {
   Inclined,
   InclinedReversed,
   Horizontal,
   HorizontalReversed,
   Vertical,
   VerticalReversed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
   etype: EdgeType,
   index: usize,
}

impl Edge {
   #[inline]
   pub fn new(etype: EdgeType, index: usize) -> Self {
      Edge {
         etype: etype,
         index: index,
      }
   }
}

impl Default for Edge {
   #[inline]
   fn default() -> Edge {
      Edge::new(EdgeType::Inclined, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct InclinedEdge {
   p1: usize,
   p2: usize,
}

impl InclinedEdge {
   #[inline]
   pub fn new(p1: usize, p2: usize) -> Self {
      InclinedEdge {
         p1: p1,
         p2: p2,
      }
   }
}

impl Default for InclinedEdge {
   #[inline]
   fn default() -> InclinedEdge {
      InclinedEdge::new(0, 0)
   }
}

pub fn fill_default<T>(capacity: usize) -> Vec<T> where T: Default + Clone {
   iter::repeat(T::default()).take(capacity).collect()
}

struct TriangleRenderer {
   triangles: Vec<Edge>,
   incl: Vec<InclinedEdge>,
   horz: Vec<i64>,
   vert: Vec<i64>,
   points: Vec<Point>,

   above: Vec<Edge>,
   below: Vec<Edge>,
   a_incl: Vec<InclinedEdge>,
   a_horz: Vec<i64>,
   a_vert: Vec<i64>,
   a_points: Vec<Point>,
}

impl TriangleRenderer {
   fn new() -> Self {
      TriangleRenderer {
         triangles: vec![],
         incl: vec![],
         horz: vec![],
         vert: vec![],
         points: vec![],

         above: fill_default(262144),
         below: fill_default(262144),
         a_incl: fill_default(262144),
         a_horz: fill_default(262144),
         a_vert: fill_default(262144),
         a_points: fill_default(262144),
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

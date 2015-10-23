#![allow(dead_code)]
extern crate polydraw;

use std::i64;
use std::cmp::min;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::geom::ring::Ring;
use polydraw::draw::RGB;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Poly {
   color: RGB,
   start: usize,
   end: usize,
}

impl Poly {
   #[inline]
   pub fn new(color: RGB, start: usize, end: usize) -> Self {
      Poly {
         color: color,
         start: start,
         end: end,
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Edge {
   Inclined(usize),
   InclinedRev(usize),
   Horizontal(i64),
   HorizontalRev(i64),
   Vertical(i64),
   VerticalRev(i64),
}

impl Default for Edge {
   #[inline]
   fn default() -> Edge {
      Edge::Inclined(0)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PolyRef {
   src: usize,
   start: usize,
   end: usize,
}

impl PolyRef {
   #[inline]
   pub fn new(src: usize, start: usize, end: usize) -> Self {
      PolyRef {
         src: src,
         start: start,
         end: end,
      }
   }
}

impl Default for PolyRef {
   #[inline]
   fn default() -> PolyRef {
      PolyRef::new(0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct InclinedEdgeRef {
   p1: usize,
   p2: usize,
   src_p1: usize,
   src_p2: usize,
}

impl InclinedEdgeRef {
   #[inline]
   pub fn new(p1: usize, p2: usize, src_p1: usize, src_p2: usize) -> Self {
      InclinedEdgeRef {
         p1: p1,
         p2: p2,
         src_p1: src_p1,
         src_p2: src_p2,
      }
   }
}

impl Default for InclinedEdgeRef {
   #[inline]
   fn default() -> InclinedEdgeRef {
      InclinedEdgeRef::new(0, 0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PolyMinYRef {
   poly: usize,
   min_y: i64,
}

struct PolySource {
   polys: Vec<Poly>,
   edges: Vec<Edge>,
   inclined: Vec<InclinedEdge>,
   points: Vec<Point>,
}

impl PolySource {
   fn new() -> Self {
      let polys = vec![
         // A
         Poly::new(RGB::new(18, 78, 230), 0, 3),
         // B
         Poly::new(RGB::new(47, 11, 206), 3, 6),
         // C
         Poly::new(RGB::new(170, 44, 206), 6, 9),
         // D
         Poly::new(RGB::new(243, 0, 149), 9, 12),
         // E
         Poly::new(RGB::new(170, 36, 14), 12, 15),
         // F
         Poly::new(RGB::new(219, 65, 18), 15, 18),
         // G
         Poly::new(RGB::new(254, 185, 21), 18, 21),
         // H
         Poly::new(RGB::new(244, 239, 114), 21, 24),
         // I
         Poly::new(RGB::new(109, 233, 158), 24, 27),
         // J
         Poly::new(RGB::new(66, 222, 241), 27, 30),
      ];

      let edges = vec![
         // 0: A
         Edge::Vertical(0),
         Edge::InclinedRev(0),
         Edge::InclinedRev(1),

         // 1: B
         Edge::Inclined(1),
         Edge::InclinedRev(2),
         Edge::InclinedRev(3),

         // 2: C
         Edge::Inclined(3),
         Edge::InclinedRev(4),
         Edge::HorizontalRev(0),

         // 3: D
         Edge::Inclined(4),
         Edge::Inclined(5),
         Edge::InclinedRev(6),

         // 4: E
         Edge::Inclined(6),
         Edge::Inclined(7),
         Edge::VerticalRev(10),

         // 5: F
         Edge::Inclined(2),
         Edge::Inclined(8),
         Edge::InclinedRev(9),

         // 6: G
         Edge::Inclined(9),
         Edge::InclinedRev(10),
         Edge::InclinedRev(5),

         // 7: H
         Edge::Inclined(0),
         Edge::InclinedRev(11),
         Edge::InclinedRev(8),

         // 8: I
         Edge::Inclined(10),
         Edge::Inclined(12),
         Edge::InclinedRev(7),

         // 9: J
         Edge::Inclined(11),
         Edge::Horizontal(10),
         Edge::InclinedRev(12),
      ];

      let inclined = vec![
         InclinedEdge::new(2, 1),  // 0
         InclinedEdge::new(0, 2),  // 1
         InclinedEdge::new(3, 2),  // 2
         InclinedEdge::new(0, 3),  // 3
         InclinedEdge::new(4, 3),  // 4
         InclinedEdge::new(3, 5),  // 5
         InclinedEdge::new(4, 5),  // 6
         InclinedEdge::new(5, 6),  // 7
         InclinedEdge::new(2, 7),  // 8
         InclinedEdge::new(3, 7),  // 9
         InclinedEdge::new(5, 7),  // 10
         InclinedEdge::new(7, 1),  // 11
         InclinedEdge::new(7, 6),  // 12
      ];

      let points = vec![
         Point::new(0, 0),   // 0
         Point::new(0, 10),  // 1
         Point::new(2, 5),   // 2
         Point::new(3, 1),   // 3
         Point::new(10, 0),  // 4
         Point::new(8, 5),   // 5
         Point::new(10, 10), // 6
         Point::new(7, 9),   // 7
      ];

      PolySource {
         polys: polys,
         edges: edges,
         inclined: inclined,
         points: points,
      }
   }

   pub fn polys_min_y(&self) -> Vec<PolyMinYRef> {
      let mut v = Vec::new();

      for (poly_index, poly) in self.polys.iter().enumerate() {
         let mut min_y = i64::MAX;

         for edge_index in poly.start..poly.end {
            let y = match self.edges[edge_index] {
               Edge::Inclined(i) => {
                  self.inclined_min_y(i)
               },
               Edge::InclinedRev(i) => {
                  self.inclined_min_y(i)
               },
               Edge::Horizontal(y) => {
                  y
               },
               Edge::HorizontalRev(y) => {
                  y
               },
               _ => {
                  continue;
               }
            };

            if y < min_y {
               min_y = y;
            }
         }

         v.push(PolyMinYRef {
            poly: poly_index,
            min_y: min_y,
         });
      }

      v.sort_by(|a, b| a.min_y.cmp(&b.min_y));

      v
   }

   #[inline]
   fn inclined_min_y(&self, i: usize) -> i64 {
      let inclined = self.inclined[i];
      min(
         self.points[inclined.p1].y,
         self.points[inclined.p2].y
      )
   }

   #[inline]
   fn min_max_x_y(&self) -> (i64, i64, i64, i64) {
      let mut min_x = i64::MAX;
      let mut max_x = i64::MIN;

      let mut min_y = i64::MAX;
      let mut max_y = i64::MIN;

      for p in &self.points {
         if p.x < min_x {
            min_x = p.x;
         }

         if p.x > max_x {
            max_x = p.x;
         }

         if p.y < min_y {
            min_y = p.y;
         }

         if p.y > max_y {
            max_y = p.y;
         }
      }

      (min_x, max_x, min_y, max_y)
   }
}

struct TriangleRenderer {
   src: PolySource,
   src_min_y: Vec<PolyMinYRef>,

   upper_polys: Ring<PolyRef>,
   upper_edges: Ring<Edge>,

   lower_polys: Ring<PolyRef>,
   lower_edges: Ring<Edge>,

   points: Ring<Point>,
   inclined: Ring<InclinedEdgeRef>,
}

impl TriangleRenderer {
   fn new() -> Self {
      let src = PolySource::new();
      let src_min_y = src.polys_min_y();

      TriangleRenderer {
         src: src,
         src_min_y: src_min_y,

         upper_polys: Ring::new(65536),
         upper_edges: Ring::new(262144),

         lower_polys: Ring::new(65536),
         lower_edges: Ring::new(262144),

         points: Ring::new(262144),
         inclined: Ring::new(262144),
      }
   }

   pub fn clear(&mut self) {
      self.upper_polys.clear();
      self.upper_edges.clear();

      self.lower_polys.clear();
      self.lower_edges.clear();

      self.points.clear();
      self.inclined.clear();
   }
}


impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.clear();
   }
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle3")
      .run();
}

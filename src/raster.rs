use std::cmp::{PartialOrd, Ordering};
use std::iter::repeat;
use std::usize;

use draw::RGB;


#[derive(Debug, PartialEq, Eq)]
pub struct Point {
   pub x: i64,
   pub y: i64,
}

impl Point {
   #[inline]
   pub fn new(x: i64, y: i64) -> Self {
      Point {
         x: x,
         y: y,
      }
   }
}

impl PartialOrd for Point {
   fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for Point {
   fn cmp(&self, other: &Point) -> Ordering {
      if self.y < other.y {
         Ordering::Less
      } else if self.y > other.y {
         Ordering::Greater
      } else if self.x < other.x {
         Ordering::Less
      } else if self.x > other.x {
         Ordering::Greater
      } else {
         Ordering::Equal
      }
   }
}

#[derive(Debug)]
pub struct Segment {
   pub p1: usize,
   pub p2: usize,
}

impl Segment {
   #[inline]
   pub fn new(p1: usize, p2: usize) -> Self {
      Segment {
         p1: p1,
         p2: p2,
      }
   }
}

#[derive(Debug)]
pub struct Circle {
   pub center: usize,
   pub radius: i64,
}

impl Circle {
   #[inline]
   pub fn new(center: usize, radius: i64) -> Self {
      Circle {
         center: center,
         radius: radius,
      }
   }
}

#[derive(Debug)]
pub enum EdgeType {
   LTR,  // line top-right
   LTL,  // line top-left
   LBR,  // line bottom-right
   LBL,  // line bottom-left

   LHR,  // line horizontal-right
   LHL,  // line horizontal-left
   LVT,  // line vertical-top
   LVB,  // line vertical-bottom

   CTR, // clockwise arc top-right
   CTL, // clockwise arc top-left
   CBR, // clockwise arc bottom-right
   CBL, // clockwise arc bottom-left

   ATR, // anti-clockwise arc top-right
   ATL, // anti-clockwise arc top-left
   ABR, // anti-clockwise arc bottom-right
   ABL, // anti-clockwise arc bottom-left
}

impl EdgeType {
   #[inline]
   pub fn reversed(&self) -> bool {
      match *self {
         EdgeType::LBR | EdgeType::LBL | EdgeType::LHL | EdgeType::LVB |
         EdgeType::CBR | EdgeType::CBL | EdgeType::ATR | EdgeType::ATL => {
            true
         },
         _ => {
            false
         }
      }
   }
}

#[derive(Debug)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub circle: usize,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, circle: usize) -> Self {
      Edge {
         edge_type: edge_type,
         segment: segment,
         circle: circle,
      }
   }

   #[inline]
   pub fn reversed(&self) -> bool {
      self.edge_type.reversed()
   }
}

#[derive(Debug)]
pub struct Poly {
   pub start: usize,
   pub end: usize,
   pub color: usize,
}

impl Poly {
   #[inline]
   pub fn new(start: usize, end: usize, color: usize) -> Self {
      Poly {
         start: start,
         end: end,
         color: color,
      }
   }
}

pub struct Scene {
   pub points: Vec<Point>,
   pub segments: Vec<Segment>,
   pub circles: Vec<Circle>,
   pub edges: Vec<Edge>,
   pub polys: Vec<Poly>,
   pub colors: Vec<RGB>,
}

impl Scene {
   pub fn check_correctness(&self) {
      for poly in &self.polys {
         self.check_poly_connected(poly);
         self.check_segments_orientation(poly);
         self.check_edges_orientation(poly);
      }

      self.check_index_coverage();
   }

   fn check_poly_connected(&self, poly: &Poly) {
      let mut prev = self.edge_head(&self.edges[poly.end - 1]);

      for edge_index in poly.start..poly.end {
         let ref edge = self.edges[edge_index];

         let current = self.edge_tail(edge);
         if current != prev {
            panic!("Unclosed poly : {:?}", poly);
         }

         prev = self.edge_head(edge);
      }
   }

   fn check_segments_orientation(&self, poly: &Poly) {
      for edge_index in poly.start..poly.end {
         let ref edge = self.edges[edge_index];
         let ref segment = self.segments[edge.segment];

         let ref less = self.points[segment.p1];
         let ref greater = self.points[segment.p2];

         if greater <= less {
            panic!("Wrong segment orientation : {:?} / {:?}", edge, segment);
         }
      }
   }

   fn check_edges_orientation(&self, poly: &Poly) {
      for edge_index in poly.start..poly.end {
         let ref edge = self.edges[edge_index];

         let tail = self.edge_tail(edge);
         let head = self.edge_head(edge);

         let reversed = edge.reversed();

         if (reversed && tail <= head) || (!reversed && head <= tail) {
            panic!("Wrong edge orientation : {:?}", edge);
         }
      }
   }

   fn check_index_coverage(&self) {
      self.check_edge_index_coverage();
      self.check_circle_index_coverage();
      self.check_segment_index_coverage();
   }

   fn check_edge_index_coverage(&self) {
      let len = self.edges.len();
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for poly in &self.polys {
         for edge_index in poly.start..poly.end {
            coverage[edge_index] = true;
         }
      }

      if coverage.contains(&false) {
         panic!("Unreferenced edge found");
      }
   }

   fn check_circle_index_coverage(&self) {
      let len = self.circles.len();
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for edge in &self.edges {
         if edge.circle != usize::MAX {
            coverage[edge.circle] = true;
         }
      }

      if coverage.contains(&false) {
         panic!("Unreferenced circle found");
      }
   }

   fn check_segment_index_coverage(&self) {
      let len = self.segments.len();
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for edge in &self.edges {
         coverage[edge.segment] = true;
      }

      if coverage.contains(&false) {
         panic!("Unreferenced segment found");
      }
   }

   // TODO: Combine into edge_head_tail method?

   fn edge_head(&self, edge: &Edge) -> &Point {
      let ref segment = self.segments[edge.segment];
      if edge.reversed() {
         &self.points[segment.p1]
      } else {
         &self.points[segment.p2]
      }
   }

   fn edge_tail(&self, edge: &Edge) -> &Point {
      let ref segment = self.segments[edge.segment];
      if edge.reversed() {
         &self.points[segment.p2]
      } else {
         &self.points[segment.p1]
      }
   }
}

use std::usize;

use geom::point::Point;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
         EdgeType::CBR | EdgeType::CBL | EdgeType::ABR | EdgeType::ABL => {
            true
         },
         _ => {
            false
         }
      }
   }
}

impl Default for EdgeType {
   fn default() -> EdgeType {
      EdgeType::LTR
   }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeSrc {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub circle: usize,
}

impl EdgeSrc {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, circle: usize) -> Self {
      EdgeSrc {
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

impl Default for EdgeSrc {
   fn default() -> EdgeSrc {
      EdgeSrc::new(EdgeType::LTR, 0, 0)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub circle: usize,
   pub p1: Point,
   pub p2: Point,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, circle: usize, p1: Point, p2: Point) -> Self {
      Edge {
         edge_type: edge_type,
         segment: segment,
         circle: circle,
         p1: p1,
         p2: p2,
      }
   }

   #[inline]
   pub fn reversed(&self) -> bool {
      self.edge_type.reversed()
   }

   #[inline]
   pub fn vert_bottom(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LVB, usize::MAX, usize::MAX, p1, p2)
   }

   #[inline]
   pub fn vert_top(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LVT, usize::MAX, usize::MAX, p1, p2)
   }

   #[inline]
   pub fn hori_left(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LHL, usize::MAX, usize::MAX, p1, p2)
   }

   #[inline]
   pub fn hori_right(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LHR, usize::MAX, usize::MAX, p1, p2)
   }
}

impl Default for Edge {
   fn default() -> Edge {
      Edge::new(EdgeType::LTR, 0, 0, Point::default(), Point::default())
   }
}

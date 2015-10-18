use super::number::NumberOps;
use super::ring::Ring;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InclinedEdge {
   pub x1: i64,
   pub y1: i64,
   pub x2: i64,
   pub y2: i64,
   pub ox1: i64,
   pub oy1: i64,
   pub ox2: i64,
   pub oy2: i64,
   pub dx: i64,
   pub dy: i64,
}

impl InclinedEdge {
   #[inline]
   pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      InclinedEdge {
         x1: x1,
         y1: y1,
         x2: x2,
         y2: y2,
         ox1: x1,
         oy1: y1,
         ox2: x2,
         oy2: y2,
         dx: x2 - x1,
         dy: y2 - y1,
      }
   }

   #[inline]
   fn h_intersect(&self, y: i64) -> i64 {
      self.ox1 + (self.dx * (y - self.oy1)).rounding_div(self.dy)
   }

   #[allow(dead_code)]
   #[inline]
   fn v_intersect(&self, x: i64) -> i64 {
      self.oy1 + (self.dy * (x - self.ox1)).rounding_div(self.dx)
   }

   #[inline]
   fn segment(&self, x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      InclinedEdge {
         x1: x1,
         y1: y1,
         x2: x2,
         y2: y2,
         ox1: self.ox1,
         oy1: self.oy1,
         ox2: self.ox2,
         oy2: self.oy2,
         dx: self.dx,
         dy: self.dy,
      }
   }
}

impl Default for InclinedEdge {
   fn default() -> InclinedEdge {
      InclinedEdge::new(0, 0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HorizontalEdge {
   pub x1: i64,
   pub x2: i64,
   pub y: i64,
}

impl HorizontalEdge {
   #[inline]
   pub fn new(x1: i64, x2: i64, y: i64) -> Self {
      HorizontalEdge {
         x1: x1,
         x2: x2,
         y: y,
      }
   }
}

impl Default for HorizontalEdge {
   fn default() -> HorizontalEdge {
      HorizontalEdge::new(0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VerticalEdge {
   pub x: i64,
   pub y1: i64,
   pub y2: i64,
}

impl VerticalEdge {
   #[inline]
   pub fn new(x: i64, y1: i64, y2: i64) -> Self {
      VerticalEdge {
         x: x,
         y1: y1,
         y2: y2,
      }
   }
}

impl Default for VerticalEdge {
   fn default() -> VerticalEdge {
      VerticalEdge::new(0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Edge {
   Inclined(InclinedEdge),
   Horizontal(HorizontalEdge),
   Vertical(VerticalEdge)
}

impl Default for Edge {
   fn default() -> Edge {
      Edge::Horizontal(
         HorizontalEdge::default()
      )
   }
}

pub fn h_split(y: i64, lower: &mut Ring<Edge>, src_upper: &mut Ring<Edge>) {
   let x1_intersect;

   let end = src_upper.end();

   let mut i = src_upper.start();

   loop { // Edge's first point below y
      match src_upper[i] {
         Edge::Inclined(inclined) => {
            if inclined.y2 < y {
               lower.push(Edge::Inclined(inclined));
            } else if inclined.y2 > y {
               x1_intersect = inclined.h_intersect(y);

               lower.push(Edge::Inclined(
                  inclined.segment(inclined.x1, inclined.y1, x1_intersect, y)
               ));

               src_upper.push(Edge::Inclined(
                  inclined.segment(x1_intersect, y, inclined.x2, inclined.y2)
               ));

               break;
            } else {
               x1_intersect = inclined.x2;

               lower.push(Edge::Inclined(inclined));

               break;
            }
         },
         Edge::Vertical(vertical) => {
            if vertical.y2 < y {
               lower.push(Edge::Vertical(vertical));
            } else if vertical.y2 > y {
               x1_intersect = vertical.x;

               lower.push(Edge::Vertical(
                  VerticalEdge::new(x1_intersect, vertical.y1, y)
               ));

               src_upper.push(Edge::Vertical(
                  VerticalEdge::new(x1_intersect, y, vertical.y2)
               ));

               break;
            } else {
               x1_intersect = vertical.x;

               lower.push(Edge::Vertical(vertical));

               break;
            }
         },
         Edge::Horizontal(horizontal) => {
            lower.push(Edge::Horizontal(horizontal));
         }
      }

      i += 1;

      if i == end {
         return;
      }
   }

   i += 1;

   loop { // Edge's first point above y
      match src_upper[i] {
         Edge::Inclined(inclined) => {
            if inclined.y2 > y {
               src_upper.push(Edge::Inclined(inclined));
            } else if inclined.y2 < y {
               let x2_intersect = inclined.h_intersect(y);

               src_upper.push(Edge::Inclined(
                  inclined.segment(inclined.x1, inclined.y1, x2_intersect, y)
               ));

               h_split_push_horizontal(
                  x1_intersect, x2_intersect, y, lower, src_upper
               );

               lower.push(Edge::Inclined(
                  inclined.segment(x2_intersect, y, inclined.x2, inclined.y2)
               ));

               break;
            } else {
               let x2_intersect = inclined.x2;

               src_upper.push(Edge::Inclined(inclined));

               h_split_push_horizontal(
                  x1_intersect, x2_intersect, y, lower, src_upper
               );

               break;
            }
         },
         Edge::Vertical(vertical) => {
            if vertical.y2 > y {
               src_upper.push(Edge::Vertical(vertical));
            } else if vertical.y2 < y {
               let x2_intersect = vertical.x;

               src_upper.push(Edge::Vertical(
                  VerticalEdge::new(x2_intersect, vertical.y1, y)
               ));

               h_split_push_horizontal(
                  x1_intersect, x2_intersect, y, lower, src_upper
               );

               lower.push(Edge::Vertical(
                  VerticalEdge::new(x2_intersect, y, vertical.y2)
               ));

               break;
            } else {
               let x2_intersect = vertical.x;

               src_upper.push(Edge::Vertical(vertical));

               h_split_push_horizontal(
                  x1_intersect, x2_intersect, y, lower, src_upper
               );

               break;
            }
         },
         Edge::Horizontal(horizontal) => {
            src_upper.push(Edge::Horizontal(horizontal));
         }
      }

      i += 1;

      if i == end {
         return;
      }
   }

   i += 1;

   for j in i..end { // Edge's first point again below y
      let edge = src_upper[j];
      lower.push(edge);
   }

   src_upper.consume_at(end);
}

#[inline]
pub fn h_split_push_horizontal(x1: i64, x2: i64, y: i64, lower: &mut Ring<Edge>, src_upper: &mut Ring<Edge>) {
   src_upper.push(Edge::Horizontal(
      HorizontalEdge::new(x2, x1, y)
   ));

   lower.push(Edge::Horizontal(
      HorizontalEdge::new(x1, x2, y)
   ));
}

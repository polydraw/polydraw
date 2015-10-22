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
      }
   }

   #[inline]
   fn h_intersect(&self, y: i64) -> i64 {
      self.ox1 + ((self.ox2 - self.ox1) * (y - self.oy1)).rounding_div(self.oy2 - self.oy1)
   }

   #[inline]
   fn v_intersect(&self, x: i64) -> i64 {
      self.oy1 + ((self.oy2 - self.oy1) * (x - self.ox1)).rounding_div(self.ox2 - self.ox1)
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

impl Edge {
   #[inline]
   pub fn x1(&self) -> i64 {
      match *self {
         Edge::Inclined(inclined) => inclined.x1,
         Edge::Horizontal(horizontal) => horizontal.x1,
         Edge::Vertical(vertical) => vertical.x,
      }
   }

   #[inline]
   pub fn y1(&self) -> i64 {
      match *self {
         Edge::Inclined(inclined) => inclined.y1,
         Edge::Horizontal(horizontal) => horizontal.y,
         Edge::Vertical(vertical) => vertical.y1,
      }
   }

   #[inline]
   pub fn x2(&self) -> i64 {
      match *self {
         Edge::Inclined(inclined) => inclined.x2,
         Edge::Horizontal(horizontal) => horizontal.x2,
         Edge::Vertical(vertical) => vertical.x,
      }
   }

   #[inline]
   pub fn y2(&self) -> i64 {
      match *self {
         Edge::Inclined(inclined) => inclined.y2,
         Edge::Horizontal(horizontal) => horizontal.y,
         Edge::Vertical(vertical) => vertical.y2,
      }
   }
}

#[inline]
pub fn h_split(y: i64, lower: &mut Ring<Edge>, src_upper: &mut Ring<Edge>) {
   let start = src_upper.start();
   let end = src_upper.end();

   if end - start <= 2 {
      src_upper.consume();
      //panic!("h_split end - start <= 2");
      return;
   }

   let double = 2 * (end - start);
   src_upper.rewind(double);
   lower.rewind(double);

   let rewinded_end = src_upper.end();

   let x1_intersect;

   let mut i = start;

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

   src_upper.consume_at(rewinded_end);
}

#[inline]
pub fn v_split(x: i64, left: &mut Ring<Edge>, src_right: &mut Ring<Edge>) {
   let start = src_right.start();
   let end = src_right.end();

   if end - start <= 2 {
      src_right.consume();
      panic!("v_split end - start <= 2");
      //return;
   }

   let double = 2 * (end - start);
   src_right.rewind(double);
   left.rewind(double);

   let rewinded_end = src_right.end();

   let y1_intersect;

   let mut i = start;

   loop { // Edge's first point below y
      match src_right[i] {
         Edge::Inclined(inclined) => {
            if inclined.x2 < x {
               left.push(Edge::Inclined(inclined));
            } else if inclined.x2 > x {
               y1_intersect = inclined.v_intersect(x);

               left.push(Edge::Inclined(
                  inclined.segment(inclined.x1, inclined.y1, x, y1_intersect)
               ));

               src_right.push(Edge::Inclined(
                  inclined.segment(x, y1_intersect, inclined.x2, inclined.y2)
               ));

               break;
            } else {
               y1_intersect = inclined.y2;

               left.push(Edge::Inclined(inclined));

               break;
            }
         },
         Edge::Horizontal(horizontal) => {
            if horizontal.x2 < x {
               left.push(Edge::Horizontal(horizontal));
            } else if horizontal.x2 > x {
               y1_intersect = horizontal.y;

               left.push(Edge::Horizontal(
                  HorizontalEdge::new(horizontal.x1, x, y1_intersect)
               ));

               src_right.push(Edge::Horizontal(
                  HorizontalEdge::new(x, horizontal.x2, y1_intersect)
               ));

               break;
            } else {
               y1_intersect = horizontal.y;

               left.push(Edge::Horizontal(horizontal));

               break;
            }
         },
         Edge::Vertical(horizontal) => {
            left.push(Edge::Vertical(horizontal));
         }
      }

      i += 1;

      if i == end {
         return;
      }
   }

   i += 1;

   loop { // Edge's first point above y
      match src_right[i] {
         Edge::Inclined(inclined) => {
            if inclined.x2 > x {
               src_right.push(Edge::Inclined(inclined));
            } else if inclined.x2 < x {
               let y2_intersect = inclined.v_intersect(x);

               src_right.push(Edge::Inclined(
                  inclined.segment(inclined.x1, inclined.y1, x, y2_intersect)
               ));

               v_split_push_vertical(
                  x, y1_intersect, y2_intersect, left, src_right
               );

               left.push(Edge::Inclined(
                  inclined.segment(x, y2_intersect, inclined.x2, inclined.y2)
               ));

               break;
            } else {
               let y2_intersect = inclined.y2;

               src_right.push(Edge::Inclined(inclined));

               v_split_push_vertical(
                  x, y1_intersect, y2_intersect, left, src_right
               );

               break;
            }
         },
         Edge::Horizontal(horizontal) => {
            if horizontal.x2 > x {
               src_right.push(Edge::Horizontal(horizontal));
            } else if horizontal.x2 < x {
               let y2_intersect = horizontal.y;

               src_right.push(Edge::Horizontal(
                  HorizontalEdge::new(horizontal.x1, x, y2_intersect)
               ));

               v_split_push_vertical(
                  x, y1_intersect, y2_intersect, left, src_right
               );

               left.push(Edge::Horizontal(
                  HorizontalEdge::new(x, horizontal.x2, y2_intersect)
               ));

               break;
            } else {
               let y2_intersect = horizontal.y;

               src_right.push(Edge::Horizontal(horizontal));

               v_split_push_vertical(
                  x, y1_intersect, y2_intersect, left, src_right
               );

               break;
            }
         },
         Edge::Vertical(vertical) => {
            src_right.push(Edge::Vertical(vertical));
         }
      }

      i += 1;

      if i == end {
         return;
      }
   }

   i += 1;

   for j in i..end { // Edge's first point again below y
      let edge = src_right[j];
      left.push(edge);
   }

   src_right.consume_at(rewinded_end);
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

#[inline]
pub fn v_split_push_vertical(x: i64, y1: i64, y2: i64, left: &mut Ring<Edge>, src_right: &mut Ring<Edge>) {
   src_right.push(Edge::Vertical(
      VerticalEdge::new(x, y2, y1)
   ));

   left.push(Edge::Vertical(
      VerticalEdge::new(x, y1, y2)
   ));
}

#[cfg(test)]
mod tests {
   use test::Bencher;

   use super::super::ring::Ring;

   use super::*;

   #[bench]
   fn bench_split2(b: &mut Bencher) {
      let mut up = Ring::new(131072);
      let mut right = Ring::new(524288);
      let mut left = Ring::new(524288);

      b.iter(|| {
         for _ in 0..1000 {
            up.push(Edge::Inclined(
               InclinedEdge::new(100, 100, 300, 600)
            ));
            up.push(Edge::Inclined(
               InclinedEdge::new(300, 600, 800, 400)
            ));
            up.push(Edge::Inclined(
               InclinedEdge::new(800, 400, 100, 100)
            ));

            h_split(200, &mut right, &mut up);

            v_split(200, &mut left, &mut right);
            left.consume();

            v_split(300, &mut left, &mut right);
            left.consume();

            right.consume();

            h_split(300, &mut right, &mut up);

            v_split(200, &mut left, &mut right);
            left.consume();

            v_split(300, &mut left, &mut right);
            left.consume();

            v_split(400, &mut left, &mut right);
            left.consume();

            v_split(500, &mut left, &mut right);
            left.consume();

            right.consume();

            h_split(400, &mut right, &mut up);

            v_split(200, &mut left, &mut right);
            left.consume();

            v_split(300, &mut left, &mut right);
            left.consume();

            v_split(400, &mut left, &mut right);
            left.consume();

            v_split(500, &mut left, &mut right);
            left.consume();

            v_split(600, &mut left, &mut right);
            left.consume();

            v_split(700, &mut left, &mut right);
            left.consume();

            right.consume();

            up.consume();
         }
      });
   }
}

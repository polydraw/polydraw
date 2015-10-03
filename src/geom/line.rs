use super::point::Point;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineIntersection {
   Point(Point),
   Overlapping,
   None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
   pub a: f64,
   pub b: f64,
   pub c: f64
}

impl Line {
   pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      let (x1, y1, x2, y2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
      Line {
         a: y2 - y1,
         b: x1 - x2,
         c: x2 * y1 - x1 * y2
      }
   }

   pub fn update(&mut self, x1: i64, y1: i64, x2: i64, y2: i64) {
      let (x1, y1, x2, y2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);

      self.a = y2 - y1;
      self.b = x1 - x2;
      self.c = x2 * y1 - x1 * y2;
   }

   pub fn intersect(&self, line: &Self) -> LineIntersection {
      let denominator = self.a * line.b - line.a * self.b;

      if denominator == 0. {
         if self == line {
            return LineIntersection::Overlapping;
         } else {
            return LineIntersection::None;
         }
      }

      let x = (line.c * self.b - self.c * line.b) / denominator;
      let y = (line.a * self.c - self.a * line.c) / denominator;

      LineIntersection::Point(Point::new(x.round() as i64, y.round() as i64))
   }

   #[inline]
   pub fn is_horizontal(&self) -> bool {
      self.a == 0.
   }

   #[inline]
   pub fn is_vertical(&self) -> bool {
      self.b == 0.
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineSegmentIntersection {
   Point(Point),
   Overlapping(Point, Point),
   None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LineSegment {
   p1: Point,
   p2: Point,
   line: Line
}

impl LineSegment {
   pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      let p1 = Point::new(x1, y1);
      let p2 = Point::new(x2, y2);
      let line = Line::new(x1, y1, x2, y2);

      LineSegment {
         p1: p1,
         p2: p2,
         line: line
      }
   }

   pub fn update(&mut self, x1: i64, y1: i64, x2: i64, y2: i64) {
      self.p1.x = x1;
      self.p1.y = y1;
      self.p2.x = x2;
      self.p2.y = y2;

      self.line.update(x1, y1, x2, y2);
   }

   #[inline]
   pub fn p1(&self) -> Point {
      self.p1
   }

   #[inline]
   pub fn p2(&self) -> Point {
      self.p2
   }

   #[inline]
   pub fn x1(&self) -> i64 {
      self.p1.x
   }

   #[inline]
   pub fn y1(&self) -> i64 {
      self.p1.y
   }

   #[inline]
   pub fn x2(&self) -> i64 {
      self.p2.x
   }

   #[inline]
   pub fn y2(&self) -> i64 {
      self.p2.y
   }

   #[inline]
   pub fn line(&self) -> &Line {
      &self.line
   }

   #[inline]
   fn is_inside(&self, p: &Point) -> bool {
      if self.line.is_vertical() {
         let (ls1_min, ls1_max) = if self.y2() > self.y1() {
            (self.p1(), self.p2())
         } else {
            (self.p2(), self.p1())
         };

         p.y >= ls1_min.y && p.y <= ls1_max.y
      } else {
         let (ls1_min, ls1_max) = if self.x2() > self.x1() {
            (self.p1(), self.p2())
         } else {
            (self.p2(), self.p1())
         };

         p.x >= ls1_min.x && p.x <= ls1_max.x
      }
   }

   pub fn intersect(&self, ls: &Self) -> LineSegmentIntersection {
      match self.line.intersect(ls.line()) {
         LineIntersection::Point(p) => {
            if self.is_inside(&p) && ls.is_inside(&p) {
               LineSegmentIntersection::Point(p)
            } else {
               LineSegmentIntersection::None
            }
         },

         LineIntersection::Overlapping => {
            let (p1, p2) = if self.line.is_vertical() {
               let (ls1_min, ls1_max) = if self.y2() > self.y1() {
                  (self.p1(), self.p2())
               } else {
                  (self.p2(), self.p1())
               };

               let (ls2_min, ls2_max) = if ls.y2() > ls.y1() {
                  (ls.p1(), ls.p2())
               } else {
                  (ls.p2(), ls.p1())
               };

               if ls1_min.y < ls2_min.y {
                  if ls1_max.y < ls2_min.y {
                     return LineSegmentIntersection::None;
                  } else if ls1_max.y < ls2_max.y {
                     (ls2_min, ls1_max)
                  } else {
                     (ls2_min, ls2_max)
                  }
               } else {
                  if ls2_max.y < ls2_min.y {
                     return LineSegmentIntersection::None;
                  } else if ls2_max.y < ls1_max.y {
                     (ls1_min, ls2_max)
                  } else {
                     (ls1_min, ls1_max)
                  }
               }
            } else {
               let (ls1_min, ls1_max) = if self.x2() > self.x1() {
                  (self.p1(), self.p2())
               } else {
                  (self.p2(), self.p1())
               };

               let (ls2_min, ls2_max) = if ls.x2() > ls.x1() {
                  (ls.p1(), ls.p2())
               } else {
                  (ls.p2(), ls.p1())
               };

               if ls1_min.x < ls2_min.x {
                  if ls1_max.x < ls2_min.x {
                     return LineSegmentIntersection::None;
                  } else if ls1_max.x < ls2_max.x {
                     (ls2_min, ls1_max)
                  } else {
                     (ls2_min, ls2_max)
                  }
               } else {
                  if ls2_max.x < ls2_min.x {
                     return LineSegmentIntersection::None;
                  } else if ls2_max.x < ls1_max.x {
                     (ls1_min, ls2_max)
                  } else {
                     (ls1_min, ls1_max)
                  }
               }
            };

            if p1 == p2 {
               LineSegmentIntersection::Point(p1)
            } else {
               LineSegmentIntersection::Overlapping(p1, p2)
            }
         }

         LineIntersection::None => LineSegmentIntersection::None,
      }
   }
}

impl Default for LineSegment {
   fn default() -> LineSegment {
      LineSegment::new(0, 0, 0, 0)
   }
}

#[cfg(test)]
mod tests {
   use super::super::point::Point;

   use super::*;

   #[test]
   fn test_line_new() {
      let ln = Line::new(
         10_000, 20_000,
         -100_000, 0
      );

      assert_eq!(ln.a, -20_000.);
      assert_eq!(ln.b, 110_000.);
      assert_eq!(ln.c, -2_000_000_000.);
   }

   #[test]
   fn test_line_intersect() {
      let l1 = Line::new(
         1, 1,
         7, 5
      );

      let l2 = Line::new(
         4, 5,
         7, 0
      );

      let intersection = l1.intersect(&l2);

      assert_eq!(intersection, LineIntersection::Point(Point::new(5, 4)));
   }

   #[test]
   fn test_line_intersect_mirror() {
      let l1 = Line::new(
         1, -1,
         7, -5
      );

      let l2 = Line::new(
         4, -5,
         7, 0
      );

      let intersection = l1.intersect(&l2);

      assert_eq!(intersection, LineIntersection::Point(Point::new(5, -4)));
   }
}

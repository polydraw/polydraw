use super::number::Number;
use super::point::Point;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineIntersection<T> {
   Point(Point<T>),
   Overlapping,
   None,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line<T> {
   a: T,
   b: T,
   c: T
}

impl<T> Line<T> where T: Number {
   pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
      Line {
         a: y2 - y1,
         b: x1 - x2,
         c: x2 * y1 - x1 * y2
      }
   }

   pub fn intersect(&self, line: &Self) -> LineIntersection<T> {
      let denominator = self.a * line.b - line.a * self.b;

      if denominator == T::zero() {
         if self == line {
            return LineIntersection::Overlapping;
         } else {
            return LineIntersection::None;
         }
      }

      let x = (line.c * self.b - self.c * line.b).rounding_div(denominator);
      let y = (line.a * self.c - self.a * line.c).rounding_div(denominator);

      LineIntersection::Point(Point::new(x, y))
   }
}

#[cfg(test)]
mod tests {
   use super::super::point::Point;

   use super::*;

   #[test]
   fn test_line_new_i64() {
      let ln = Line::new(
         10_000_i64, 20_000_i64,
         -100_000_i64, 0_i64
      );

      assert_eq!(ln.a, -20_000);
      assert_eq!(ln.b, 110_000);
      assert_eq!(ln.c, -2_000_000_000);
   }

   #[test]
   fn test_line_intersect_i64() {
      let l1 = Line::new(
         1_i64, 1_i64,
         7_i64, 5_i64
      );

      let l2 = Line::new(
         4_i64, 5_i64,
         7_i64, 0_i64
      );

      let intersection = l1.intersect(&l2);

      assert_eq!(intersection, LineIntersection::Point(Point::new(5_i64, 4_i64)));
   }

   #[test]
   fn test_line_intersect_mirror_i64() {
      let l1 = Line::new(
         1_i64, -1_i64,
         7_i64, -5_i64
      );

      let l2 = Line::new(
         4_i64, -5_i64,
         7_i64, 0_i64
      );

      let intersection = l1.intersect(&l2);

      assert_eq!(intersection, LineIntersection::Point(Point::new(5_i64, -4_i64)));
   }
}

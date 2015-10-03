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

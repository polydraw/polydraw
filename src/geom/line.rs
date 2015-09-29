use super::coordinate::Coordinate;
use super::point::Point;

pub enum LineIntersection<T> {
   Point(Point<T>),
   Parallel,
   Overlapping,
}

pub struct Line<T> {
   pub a: T,
   pub b: T,
   pub c: T
}

impl<T> Line<T> where T: Coordinate {
   pub fn new(p1: &Point<T>, p2: &Point<T>) -> Self {
      let a = p2.y - p1.y;
      let b = p1.x - p2.x;
      let c = -(p1.x * p2.y - p2.x * p1.y);
      Line {
         a: a,
         b: b,
         c: c
      }
   }

   pub fn intersect(&self, line: &Self) -> LineIntersection<T> {
      let denominator = self.a * line.b - line.a * self.b;

      if denominator == T::zero() {
         return LineIntersection::Overlapping;
      }

      let x = (line.c * self.b - self.c * line.b) / denominator;
      let y = (line.a * self.c - self.a * line.c) / denominator;

      LineIntersection::Point(Point::new(x, y))
   }
}

pub struct LineSegment<T> {
   pub p1: Point<T>,
   pub p2: Point<T>,
   pub line: Line<T>
}

impl<T> LineSegment<T> where T: Coordinate {
   pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
      let line = Line::new(&p1, &p2);

      LineSegment {
         p1: p1,
         p2: p2,
         line: line
      }
   }
}

#[cfg(test)]
mod tests {
   use super::LineSegment;
   use super::super::point::Point;

   #[test]
   fn test_new_f64() {
      let line = LineSegment::new(
         Point::new(10_f64, 20_f64),
         Point::new(-100_f64, 0_f64)
      );
      assert_eq!(line.p1.x, 10_f64);
      assert_eq!(line.p1.y, 20_f64);
      assert_eq!(line.p2.x, -100_f64);
      assert_eq!(line.p2.y, 0_f64);
   }

   #[test]
   fn test_new_f32() {
      let line = LineSegment::new(
         Point::new(10_f32, 20_f32),
         Point::new(-100_f32, 0_f32)
      );
      assert_eq!(line.p1.x, 10_f32);
      assert_eq!(line.p1.y, 20_f32);
      assert_eq!(line.p2.x, -100_f32);
      assert_eq!(line.p2.y, 0_f32);
   }
}

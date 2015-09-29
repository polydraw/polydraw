use super::number::Number;
use super::point::Point;

pub enum LineIntersection<T> {
   Point(Point<T>),
   Parallel,
   Overlapping,
}

#[derive(Debug)]
pub struct Line<T> {
   a: T,
   b: T,
   c: T
}

impl<T> Line<T> where T: Number {
   pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
      let (a, b, c) = Self::calc_abc(x1, y1, x2, y2);
      Line {
         a: a,
         b: b,
         c: c
      }
   }

   pub fn update(&mut self, x1: T, y1: T, x2: T, y2: T) {
      let (a, b, c) = Self::calc_abc(x1, y1, x2, y2);
      self.a = a;
      self.b = b;
      self.c = c;
   }

   #[inline]
   fn calc_abc(x1: T, y1: T, x2: T, y2: T) -> (T, T, T) {
      let a = y2 - y1;
      let b = x1 - x2;
      let c = -(x1 * y2 - x2 * y1);
      (a, b, c)
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

#[derive(Debug)]
pub struct LineSegment<T> {
   p1: Point<T>,
   p2: Point<T>,
   line: Line<T>
}

impl<T> LineSegment<T> where T: Number {
   pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
      let p1 = Point::new(x1, y1);
      let p2 = Point::new(x2, y2);
      let line = Line::new(x1, y1, x2, y2);

      LineSegment {
         p1: p1,
         p2: p2,
         line: line
      }
   }

   pub fn update(&mut self, x1: T, y1: T, x2: T, y2: T) {
      self.p1.x = x1;
      self.p1.y = y1;
      self.p2.x = x2;
      self.p2.y = y2;

      self.line.update(x1, y1, x2, y2);
   }

   #[inline]
   pub fn x1(&self) -> T {
      self.p1.x
   }

   #[inline]
   pub fn y1(&self) -> T {
      self.p1.y
   }

   #[inline]
   pub fn x2(&self) -> T {
      self.p2.x
   }

   #[inline]
   pub fn y2(&self) -> T {
      self.p2.y
   }

   #[inline]
   pub fn line(&self) -> &Line<T> {
      &self.line
   }
}

impl<T> Default for LineSegment<T> where T: Number {
   fn default() -> LineSegment<T> {
      LineSegment::new(T::zero(), T::zero(), T::zero(), T::zero())
   }
}

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::*;

   #[test]
   fn test_new_f64() {
      let line = LineSegment::new(
         10_f64, 20_f64,
         -100_f64, 0_f64
      );
      assert_eq!(line.p1.x, 10_f64);
      assert_eq!(line.p1.y, 20_f64);
      assert_eq!(line.p2.x, -100_f64);
      assert_eq!(line.p2.y, 0_f64);
   }

   #[test]
   fn test_new_f32() {
      let line = LineSegment::new(
         10_f32, 20_f32,
         -100_f32, 0_f32
      );
      assert_eq!(line.p1.x, 10_f32);
      assert_eq!(line.p1.y, 20_f32);
      assert_eq!(line.p2.x, -100_f32);
      assert_eq!(line.p2.y, 0_f32);
   }

   #[bench]
   fn bench_line_new_f64(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               Line::new(5_f64, 7_f64, 3_f64, 2_f64)
            );
         }
      });
   }
}

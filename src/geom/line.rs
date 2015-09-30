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
      let mut a = y2 - y1;
      let mut b = x1 - x2;
      let mut c = -(x1 * y2 - x2 * y1);

      let mut norm = (T::one() / (a * a + b * b)).sqrt();
      if a != T::zero() {
         if a < T::zero() {
            norm = -norm;
         }
      } else {
         if b < T::zero() {
            norm = -norm;
         }
      }

      a = a * norm;
      b = b * norm;
      c = c * norm;

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
   fn test_segment_new_f64() {
      let ls = LineSegment::new(
         10_f64, 20_f64,
         -100_f64, 0_f64
      );
      assert_eq!(ls.p1.x, 10_f64);
      assert_eq!(ls.p1.y, 20_f64);
      assert_eq!(ls.p2.x, -100_f64);
      assert_eq!(ls.p2.y, 0_f64);
   }

   #[test]
   fn test_segment_new_f32() {
      let ls = LineSegment::new(
         10_f32, 20_f32,
         -100_f32, 0_f32
      );
      assert_eq!(ls.p1.x, 10_f32);
      assert_eq!(ls.p1.y, 20_f32);
      assert_eq!(ls.p2.x, -100_f32);
      assert_eq!(ls.p2.y, 0_f32);
   }

   #[test]
   fn test_line_new_f64() {
      let ln = Line::new(
         10_f64, 20_f64,
         -100_f64, 0_f64
      );
      assert_eq!(ln.a, 0.1788854381999832_f64);
      assert_eq!(ln.b, -0.9838699100999075_f64);
      assert_eq!(ln.c, 17.88854381999832_f64);
   }

   #[test]
   fn test_line_new_f32() {
      let ln = Line::new(
         10_f32, 20_f32,
         -100_f32, 0_f32
      );
      assert_eq!(ln.a, 0.1788854381999832_f32);
      assert_eq!(ln.b, -0.9838699100999075_f32);
      assert_eq!(ln.c, 17.88854381999832_f32);
   }

   #[test]
   fn test_vline_f64() {
      let ln = Line::new(
         50_f64, 100_f64,
         50_f64, 200_f64
      );
      assert_eq!(ln.a, 1_f64);
      assert_eq!(ln.b, 0_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_vline_f32() {
      let ln = Line::new(
         50_f32, 100_f32,
         50_f32, 200_f32
      );
      assert_eq!(ln.a, 1_f32);
      assert_eq!(ln.b, 0_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[test]
   fn test_vline_one_f64() {
      let ln = Line::new(
         50_f64, 100_f64,
         50_f64, 101_f64
      );
      assert_eq!(ln.a, 1_f64);
      assert_eq!(ln.b, 0_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_vline_one_f32() {
      let ln = Line::new(
         50_f32, 100_f32,
         50_f32, 101_f32
      );
      assert_eq!(ln.a, 1_f32);
      assert_eq!(ln.b, 0_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[test]
   fn test_vline_one_down_f64() {
      let ln = Line::new(
         50_f64, 101_f64,
         50_f64, 100_f64
      );
      assert_eq!(ln.a, 1_f64);
      assert_eq!(ln.b, 0_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_vline_one_down_f32() {
      let ln = Line::new(
         50_f32, 101_f32,
         50_f32, 100_f32
      );
      assert_eq!(ln.a, 1_f32);
      assert_eq!(ln.b, 0_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[test]
   fn test_hline_f64() {
      let ln = Line::new(
         100_f64, 50_f64,
         200_f64, 50_f64
      );
      assert_eq!(ln.a, 0_f64);
      assert_eq!(ln.b, 1_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_hline_f32() {
      let ln = Line::new(
         100_f32, 50_f32,
         200_f32, 50_f32
      );
      assert_eq!(ln.a, 0_f32);
      assert_eq!(ln.b, 1_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[test]
   fn test_hline_one_f64() {
      let ln = Line::new(
         100_f64, 50_f64,
         101_f64, 50_f64
      );
      assert_eq!(ln.a, 0_f64);
      assert_eq!(ln.b, 1_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_hline_one_f32() {
      let ln = Line::new(
         100_f32, 50_f32,
         101_f32, 50_f32
      );
      assert_eq!(ln.a, 0_f32);
      assert_eq!(ln.b, 1_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[test]
   fn test_hline_one_down_f64() {
      let ln = Line::new(
         101_f64, 50_f64,
         100_f64, 50_f64
      );
      assert_eq!(ln.a, 0_f64);
      assert_eq!(ln.b, 1_f64);
      assert_eq!(ln.c, -50_f64);
   }

   #[test]
   fn test_hline_one_down_f32() {
      let ln = Line::new(
         101_f32, 50_f32,
         100_f32, 50_f32
      );
      assert_eq!(ln.a, 0_f32);
      assert_eq!(ln.b, 1_f32);
      assert_eq!(ln.c, -50_f32);
   }

   #[bench]
   fn bench_line_new_f64(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               Line::new(
                  black_box(5_f64),
                  black_box(7_f64),
                  black_box(3_f64),
                  black_box(2_f64)
               )
            );
         }
      });
   }

   #[bench]
   fn bench_line_new_f32(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               Line::new(
                  black_box(5_f32),
                  black_box(7_f32),
                  black_box(3_f32),
                  black_box(2_f32)
               )
            );
         }
      });
   }
}


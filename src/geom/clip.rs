use std::iter::repeat;
use std::ops::{Index, RangeFull};
use std::fmt::Debug;

use super::point::Point;
use super::number::NumberOps;

#[derive(Debug)]
pub struct Ring<T> {
   v: Vec<T>,
   start: usize,
   end: usize,
}

impl<T> Ring<T> where T: Default + Clone + Debug {
   #[inline]
   pub fn new(capacity: usize) -> Self {
      let v = repeat(T::default()).take(capacity).collect();

      Ring { v: v, start: 0, end: 0 }
   }

   #[inline]
   pub fn len(&self) -> usize {
      self.end - self.start
   }

   #[inline]
   pub fn last(&self) -> Option<&T> {
      self.v.last()
   }

   #[inline]
   pub fn start(&self) -> usize {
      self.start
   }

   #[inline]
   pub fn end(&self) -> usize {
      self.end
   }

   #[inline]
   pub fn consume(&mut self, marker: usize) {
      self.start = marker;
   }

   #[inline]
   pub fn reserve(&mut self, additional: usize) {
      self.v.reserve(additional);
   }

   #[inline]
   pub fn check_rewind(&mut self, extra: usize) {
      if extra > (self.v.len() - self.end) {
         if self.start != self.end {
            for i in 0..(self.end - self.start) {
               self.v[i] = self.v[self.start + i].clone();
            }
         }

         self.end -= self.start;
         self.start = 0;
      }
   }

   #[inline]
   pub fn push(&mut self, value: T) {
      self.v[self.end] = value;
      self.end += 1;
   }
}

impl<T> Index<usize> for Ring<T> {
   type Output = T;

   #[inline]
   fn index(&self, index: usize) -> &T {
      &self.v[index]
   }
}

impl<T> Index<RangeFull> for Ring<T> {
   type Output = [T];

   #[inline]
   fn index(&self, _index: RangeFull) -> &[T] {
      &self.v[self.start..self.end]
   }
}

#[inline]
pub fn h_down_intersect(y: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
   if p2.y > y {
      if p1.y < y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
   } else {
      if p1.y > y && p2.y < y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
      points.push(p2);
   }
}

#[inline]
pub fn h_up_intersect(y: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
   if p2.y < y {
      if p1.y > y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
   } else {
      if p1.y < y && p2.y > y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
      points.push(p2);
   }
}

#[inline]
pub fn v_left_intersect(x: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
   if p2.x > x {
      if p1.x < x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
   } else {
      if p1.x > x && p2.x < x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
      points.push(p2);
   }
}

#[inline]
pub fn v_right_intersect(x: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
   if p2.x < x {
      if p1.x > x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
   } else {
      if p1.x < x && p2.x > x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
      points.push(p2);
   }
}

#[inline]
pub fn h_intersect(y: i64, p1: Point, p2: Point) -> i64 {
   p1.x + ((p2.x - p1.x) * (y - p1.y)).rounding_div(p2.y - p1.y)
}

#[inline]
pub fn v_intersect(x: i64, p1: Point, p2: Point) -> i64 {
   p1.y + ((p2.y - p1.y) * (x - p1.x)).rounding_div(p2.x - p1.x)
}

pub fn hv_clip<F>(intersect: F, y: i64, points: &mut Ring<Point>) where
   F: Fn(i64, Point, Point, &mut Ring<Point>),
{
   let start = points.start();
   let end = points.end();

   points.check_rewind(2 * (end - start));

   let rewinded_end = points.end();

   let mut p1 = points[end-1];

   for i in start..end {
      let p2 = points[i];

      intersect(y, p1, p2, points);

      p1 = p2;
   }

   points.consume(rewinded_end);
}

pub fn h_split(y: i64, start: usize, down: &mut Vec<Point>, up: &mut Vec<Point>) {
   let end = up.len();

   assert!(end - start > 2);

   let mut p1 = up[end-1];

   for i in start..end {
      let p2 = up[i];

      h_split_edge(y, p1, p2, down, up);

      p1 = p2;
   }
}

pub fn v_split(x: i64, start: usize, left: &mut Vec<Point>, right: &mut Vec<Point>) {
   let end = right.len();

   assert!(end - start > 2);

   let mut p1 = right[end-1];

   for i in start..end {
      let p2 = right[i];

      v_split_edge(x, p1, p2, left, right);

      p1 = p2;
   }
}

#[inline]
pub fn h_split_edge(y: i64, p1: Point, p2: Point, down: &mut Vec<Point>, up: &mut Vec<Point>) {
   if p2.y > y {
      if p1.y < y {
         let intersection = Point::new(h_intersect(y, p1, p2), y);
         down.push(intersection);
         up.push(intersection);
      }
      up.push(p2);
   } else if p2.y < y {
      if p1.y > y {
         let intersection = Point::new(h_intersect(y, p1, p2), y);
         down.push(intersection);
         up.push(intersection);
      }
      down.push(p2);
   } else {
      down.push(p2);
      up.push(p2);
   }
}

#[inline]
pub fn v_split_edge(x: i64, p1: Point, p2: Point, left: &mut Vec<Point>, right: &mut Vec<Point>) {
   if p2.x > x {
      if p1.x < x {
         let intersection = Point::new(x, v_intersect(x, p1, p2));
         left.push(intersection);
         right.push(intersection);
      }
      right.push(p2);
   } else if p2.x < x {
      if p1.x > x {
         let intersection = Point::new(x, v_intersect(x, p1, p2));
         left.push(intersection);
         right.push(intersection);
      }
      left.push(p2);
   } else {
      left.push(p2);
      right.push(p2);
   }
}

#[cfg(test)]
mod tests {
   use test::Bencher;

   use super::super::point::Point;

   use super::*;

   #[bench]
   fn bench_clip(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            let mut points = Ring::new(30);

            points.push(Point::new(50, 50));
            points.push(Point::new(100, 200));
            points.push(Point::new(150, 100));

            hv_clip(h_down_intersect, 150, &mut points);

            hv_clip(h_up_intersect, 80, &mut points);

            hv_clip(v_right_intersect, 70, &mut points);

            hv_clip(v_left_intersect, 140, &mut points);
         }
      });
   }
/*
   #[test]
   fn test_clip() {
      let mut points = Ring::new(30);

      points.push(Point::new(50, 50));
      points.push(Point::new(100, 200));
      points.push(Point::new(150, 100));

      hv_clip(h_down_intersect, 150, &mut points);

      hv_clip(h_up_intersect, 80, &mut points);

      hv_clip(v_right_intersect, 70, &mut points);

      hv_clip(v_left_intersect, 140, &mut points);

      assert_eq!(points[..].len(), 7);

      assert_eq!(&points[..], [
         Point::new(140, 95),
         Point::new(110, 80),
         Point::new(70, 80),
         Point::new(70, 110),
         Point::new(83, 150),
         Point::new(125, 150),
         Point::new(140, 120),
      ]);
   }
*/
}

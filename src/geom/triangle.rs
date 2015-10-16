use std::mem;

use super::point::Point;

use super::number::NumberOps;

#[derive(Debug)]
pub struct Triangle {
   pub a: Point,
   pub b: Point,
   pub c: Point
}

impl Triangle {
   pub fn new(a: Point, b: Point, c: Point) -> Self {
      Triangle {
         a: a,
         b: b,
         c: c
      }
   }

   #[inline]
   pub fn clockwise(&self) -> bool {
      (self.b.x - self.a.x) * (self.c.y - self.a.y) < (self.c.x - self.a.x) * (self.b.y - self.a.y)
   }

   #[inline]
   pub fn change_orientation(&mut self) {
      mem::swap(&mut self.b, &mut self.c);
   }

   #[inline]
   pub fn centroid(&self) -> Point {
      Point::new(
         (self.a.x + self.b.x + self.c.x).rounding_div(3),
         (self.a.y + self.b.y + self.c.y).rounding_div(3)
      )
   }
}

use super::point::Point;
use super::number::NumberOps;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Circle {
   pub center: Point,
   pub radius: i64,
}

impl Circle {
   #[inline]
   pub fn new(center: Point, radius: i64) -> Self {
      Circle {
         center: center,
         radius: radius,
      }
   }

   #[inline]
   pub fn y_tr(&self, x: i64) -> i64 { // top-right
      self.center.y + self.other_delta(x - self.center.x)
   }

   #[inline]
   pub fn x_tr(&self, y: i64) -> i64 { // top-right
      self.center.x + self.other_delta(y - self.center.y)
   }

   #[inline]
   pub fn y_br(&self, x: i64) -> i64 { // bottom-right
      self.center.y - self.other_delta(x - self.center.x)
   }

   #[inline]
   pub fn x_br(&self, y: i64) -> i64 { // bottom-right
      self.center.x + self.other_delta(self.center.y - y)
   }

   #[inline]
   pub fn y_tl(&self, x: i64) -> i64 { // top-left
      self.center.y + self.other_delta(self.center.x - x)
   }

   #[inline]
   pub fn x_tl(&self, y: i64) -> i64 { // top-left
      self.center.x - self.other_delta(y - self.center.y)
   }

   #[inline]
   pub fn y_bl(&self, x: i64) -> i64 { // bottom-left
      self.center.y - self.other_delta(self.center.x - x)
   }

   #[inline]
   pub fn x_bl(&self, y: i64) -> i64 { // bottom-left
      self.center.x - self.other_delta(self.center.y - y)
   }

   #[inline]
   fn other_delta(&self, other: i64) -> i64 {
      assert!(other > 0);
      assert!(other < self.radius);
      (self.radius.pow(2) - other.pow(2)).sqrt()
   }
}

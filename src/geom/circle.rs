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
   fn other_delta(&self, delta: i64) -> i64 {
      assert!(delta > 0);
      assert!(delta < self.radius);
      (self.radius.pow(2) - delta.pow(2)).sqrt()
   }
}


#[cfg(test)]
mod tests {
   use super::super::point::Point;

   use super::*;

   #[test]
   fn test_y_tr() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let y = circle.y_tr(120_000);
      assert_eq!(y, 145_826);
   }

   #[test]
   fn test_x_tr() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let x = circle.x_tr(120_000);
      assert_eq!(x, 145_826);
   }

   #[test]
   fn test_y_br() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let y = circle.y_br(120_000);
      assert_eq!(y, 54_174);
   }

   #[test]
   fn test_x_br() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let x = circle.x_br(60_000);
      assert_eq!(x, 130_000);
   }

   #[test]
   fn test_y_tl() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let y = circle.y_tl(60_000);
      assert_eq!(y, 130_000);
   }

   #[test]
   fn test_x_tl() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let x = circle.x_tl(120_000);
      assert_eq!(x, 54_174);
   }

   #[test]
   fn test_y_bl() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let y = circle.y_bl(60_000);
      assert_eq!(y, 70_000);
   }

   #[test]
   fn test_x_bl() {
      let circle = Circle::new(Point::new(100_000, 100_000), 50_000);
      let x = circle.x_bl(60_000);
      assert_eq!(x, 70_000);
   }
}

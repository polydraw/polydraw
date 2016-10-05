use std::fmt;
use std::cmp::{PartialOrd, Ordering};
use std::ops::{Mul, MulAssign};


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Point {
   pub x: i64,
   pub y: i64
}

impl Point {
   pub fn new(x: i64, y: i64) -> Self {
      Point {
         x: x,
         y: y
      }
   }

   pub fn update(&mut self, x: i64, y: i64) {
      self.x = x;
      self.y = y;
   }
}

impl Default for Point {
   fn default() -> Point {
      Point::new(0, 0)
   }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, _rhs: Point) {
      self.x *= _rhs.x;
      self.y *= _rhs.y;
    }
}

impl PartialOrd for Point {
   fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for Point {
   fn cmp(&self, other: &Point) -> Ordering {
      if self.y < other.y {
         Ordering::Less
      } else if self.y > other.y {
         Ordering::Greater
      } else if self.x < other.x {
         Ordering::Less
      } else if self.x > other.x {
         Ordering::Greater
      } else {
         Ordering::Equal
      }
   }
}

impl Mul<i64> for Point {
   type Output = Point;

   fn mul(self, val: i64) -> Point {
      Point {
         x: self.x * val,
         y: self.y * val,
      }
   }
}

impl fmt::Debug for Point {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{} {}>", self.x, self.y)
   }
}

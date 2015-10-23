use std::cmp::{PartialOrd, Ordering};
use std::ops::MulAssign;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl PartialOrd for Point {
   fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl MulAssign<i64> for Point {
   fn mul_assign(&mut self, _rhs: i64) {
      self.x *= _rhs;
      self.y *= _rhs;
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

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::*;

   #[bench]
   fn bench_update(b: &mut Bencher) {
      let mut p = Point::new(5, 7);
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               p.update(
                  black_box(3), 8
               )
            );
         }
      });
   }
}

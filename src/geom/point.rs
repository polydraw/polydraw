use std::ops::{Add, Sub, Mul};

use super::float::FloatMath;

pub struct Point<T> {
   x: T,
   y: T
}

impl<T> Point<T> where T:
   Add<T, Output=T> +
   Sub<T, Output=T> +
   Mul<T, Output=T> +
   FloatMath +
   Default + Copy + Clone
{
   pub fn new(x: T, y: T) -> Self {
      Point {
         x: x,
         y: y
      }
   }

   pub fn distance(&self, other: &Point<T>) -> T {
      let x = other.x - self.x;
      let y = other.y - self.y;
      return (x * x + y * y).sqrt()
   }
}

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::*;

   #[bench]
   fn bench_distance_f64(b: &mut Bencher) {
      let p1 = Point::new(5_f64, 7_f64);
      let p2 = Point::new(3_f64, 2_f64);
      b.iter(|| {
         for _ in 0..black_box(100) {
            p1.distance(&p2);
         }
      });
   }
}

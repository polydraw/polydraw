use super::number::Number;
use super::distance::Distance;

#[derive(Debug)]
pub struct Point<T> {
   pub x: T,
   pub y: T
}

impl<T> Point<T> where T: Number {
   pub fn new(x: T, y: T) -> Self {
      Point {
         x: x,
         y: y
      }
   }
}

impl<T> Distance<Point<T>, T> for Point<T> where T: Number {
   fn distance(&self, other: &Self) -> T {
      let x = other.x - self.x;
      let y = other.y - self.y;
      return (x * x + y * y).sqrt()
   }
}

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::super::distance::Distance;

   use super::*;

   #[bench]
   fn bench_distance_f64(b: &mut Bencher) {
      let p1 = Point::new(5_f64, 7_f64);
      let p2 = Point::new(3_f64, 2_f64);
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               p1.distance(
                  black_box(&p2)
               )
            );
         }
      });
   }

   #[bench]
   fn bench_distance_f32(b: &mut Bencher) {
      let p1 = Point::new(5_f32, 7_f32);
      let p2 = Point::new(3_f32, 2_f32);
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               p1.distance(
                  black_box(&p2)
               )
            );
         }
      });
   }
}

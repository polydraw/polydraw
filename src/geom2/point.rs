use super::number::Number;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

   pub fn update(&mut self, x: T, y: T) {
      self.x = x;
      self.y = y;
   }
}

impl<T> Default for Point<T> where T: Number {
   fn default() -> Point<T> {
      Point::new(T::zero(), T::zero())
   }
}

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::*;

   #[bench]
   fn bench_update_i32(b: &mut Bencher) {
      let mut p = Point::new(5_i32, 7_i32);
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               p.update(
                  black_box(3_i32), 8_i32
               )
            );
         }
      });
   }

   #[bench]
   fn bench_update_i64(b: &mut Bencher) {
      let mut p = Point::new(5_i64, 7_i64);
      b.iter(|| {
         for _ in 0..1000 {
            black_box(
               p.update(
                  black_box(3_i64), 8_i64
               )
            );
         }
      });
   }
}


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

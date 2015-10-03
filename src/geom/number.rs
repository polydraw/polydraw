pub trait NumberOps : Sized {
   fn rounding_div(self, other: Self) -> Self;
}

impl NumberOps for i64 {
   #[inline]
   fn rounding_div(self, other: i64) -> i64 {
      if (self < 0) ^ (other < 0) {
         (self - (other / 2)) / other
      } else {
         (self + (other / 2)) / other
      }
   }
}

#[cfg(test)]
mod tests {
   use test::{Bencher, black_box};

   use super::*;

   #[bench]
   fn bench_rounding_div(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(1900000_i64.rounding_div(black_box(700000_i64)));
         }
      });
   }
}

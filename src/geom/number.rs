pub trait NumberOps : Sized {
   fn rounding_div(self, other: Self) -> Self;

   fn sqrt(self) -> Self;
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

   #[inline]
   fn sqrt(self) -> i64 {
      (self as f64).sqrt().round() as i64
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
            black_box(15023081_i64.rounding_div(black_box(108877_i64)));
         }
      });
   }
}

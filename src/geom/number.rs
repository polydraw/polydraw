pub trait NumberOps : Sized {
   fn rounding_div(self, other: Self) -> Self;

   fn sqrt(self) -> Self;

   fn isqrt(self) -> Self;
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

   fn isqrt(self) -> i64 {
      let mut squaredbit = ((!0_u64 >> 1) & !(!0_u64 >> 2)) as i64;
      let mut remainder = self;
      let mut root = 0;

      while squaredbit > 0 {
         if remainder >= (squaredbit | root) {
            remainder -= squaredbit | root;
            root >>= 1;
            root |= squaredbit;
         } else {
            root >>= 1;
         }
         squaredbit >>= 2;
      }

      root
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

pub trait NumberOps : Sized {
   fn rounding_div(self, other: Self) -> Self;

   fn rounding_idiv(self, other: Self) -> Self;

   fn sqrt(self) -> Self;

   fn isqrt(self) -> Self;
}

impl NumberOps for i64 {
   #[inline]
   fn rounding_div(self, other: i64) -> i64 {
      (self as f64 / other as f64).round() as i64
   }

   #[inline]
   fn rounding_idiv(self, other: i64) -> i64 {
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

   #[inline]
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
   use std::i64;

   use super::*;

   #[inline]
   pub fn rand_i64(seed: &mut i64) -> i64 {
       *seed = seed.wrapping_mul(827329174364988385).wrapping_add(1235464122344149);
       seed.rotate_left(32)
   }

   #[test]
   fn test_rounding_div() {
      let mut seed: i64 = 783437865;

      for _ in 0..1_000_000 {
         let mut left = rand_i64(&mut seed);

         if left > i64::MAX / 2 || left < i64::MIN / 2 {
            left /= 2;
         }

         let mut right = rand_i64(&mut seed);

         if right > i64::MAX / 2 || right < i64::MIN / 2 {
            right /= 2;
         }

         assert_eq!(
            left.rounding_div(right), left.rounding_idiv(right)
         );
      }
   }
}

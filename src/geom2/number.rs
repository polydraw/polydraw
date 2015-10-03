use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::{PartialEq, PartialOrd};
use std::num::{Zero, One};


pub trait NumberOps : Sized {
   fn rounding_div(self, other: Self) -> Self;
}

pub trait Number:
   NumberOps +
   Add<Self, Output=Self> +
   Sub<Self, Output=Self> +
   Mul<Self, Output=Self> +
   Div<Self, Output=Self> +
   Neg<Output=Self> +
   PartialEq +
   PartialOrd +
   Zero +
   One +
   Default +
   Copy +
   Clone {}

impl Number for i64 {}

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

impl Number for i32 {}

impl NumberOps for i32 {
   #[inline]
   fn rounding_div(self, other: i32) -> i32 {
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
   fn bench_rounding_div_i64(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(1900000_i64.rounding_div(black_box(700000_i64)));
         }
      });
   }

   #[bench]
   fn bench_rounding_div_i32(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            black_box(1900000_i32.rounding_div(black_box(700000_i32)));
         }
      });
   }
}

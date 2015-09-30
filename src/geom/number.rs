use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::{PartialEq, PartialOrd};
use std::num::{Zero, One};
use std::intrinsics;

pub trait NumberOps : Sized {
   fn sqrt(self) -> Self;

   fn powi(self, n: i32) -> Self;
}

impl NumberOps for f64 {
   #[inline]
   fn sqrt(self) -> f64 {
      unsafe { intrinsics::sqrtf64(self) }
   }

   #[inline]
   fn powi(self, n: i32) -> f64 {
      unsafe { intrinsics::powif64(self, n) }
   }
}

impl NumberOps for f32 {
   #[inline]
   fn sqrt(self) -> f32 {
      unsafe { intrinsics::sqrtf32(self) }
   }

   #[inline]
   fn powi(self, n: i32) -> f32 {
      unsafe { intrinsics::powif32(self, n) }
   }
}

pub trait Number:
   Add<Self, Output=Self> +
   Sub<Self, Output=Self> +
   Mul<Self, Output=Self> +
   Div<Self, Output=Self> +
   Neg<Output=Self> +
   PartialEq +
   PartialOrd +
   NumberOps +
   Zero +
   One +
   Default +
   Copy +
   Clone {}

impl Number for f64 {}

impl Number for f32 {}

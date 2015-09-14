use std::intrinsics;

pub trait FloatMath : Sized {
   fn sqrt(self) -> Self;
}

impl FloatMath for f64 {
   #[inline]
   fn sqrt(self) -> f64 {
      unsafe { intrinsics::sqrtf64(self) }
   }
}

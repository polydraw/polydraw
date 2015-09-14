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

impl FloatMath for f32 {
   #[inline]
   fn sqrt(self) -> f32 {
      unsafe { intrinsics::sqrtf32(self) }
   }
}

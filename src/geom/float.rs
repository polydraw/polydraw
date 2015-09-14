use std::intrinsics;

pub trait FloatOps : Sized {
   fn sqrt(self) -> Self;
}

impl FloatOps for f64 {
   #[inline]
   fn sqrt(self) -> f64 {
      unsafe { intrinsics::sqrtf64(self) }
   }
}

impl FloatOps for f32 {
   #[inline]
   fn sqrt(self) -> f32 {
      unsafe { intrinsics::sqrtf32(self) }
   }
}

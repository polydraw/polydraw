use std::intrinsics;

pub trait NumberOps : Sized {
   fn sqrt(self) -> Self;
}

impl NumberOps for f64 {
   #[inline]
   fn sqrt(self) -> f64 {
      unsafe { intrinsics::sqrtf64(self) }
   }
}

impl NumberOps for f32 {
   #[inline]
   fn sqrt(self) -> f32 {
      unsafe { intrinsics::sqrtf32(self) }
   }
}

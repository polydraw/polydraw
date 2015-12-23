
#[derive(Debug, Clone, Copy)]
pub struct Circle {
   pub center: usize,
   pub radius: i64,
}

impl Circle {
   #[inline]
   pub fn new(center: usize, radius: i64) -> Self {
      Circle {
         center: center,
         radius: radius,
      }
   }
}

impl Default for Circle {
   fn default() -> Circle {
      Circle::new(0, 0)
   }
}

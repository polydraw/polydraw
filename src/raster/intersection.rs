
#[derive(Debug, Clone)]
pub struct IntersectionRef {
   pub first_px: i64,
   pub start: usize,
   pub end: usize,
}

impl IntersectionRef {
   #[inline]
   pub fn new(first_px: i64, start: usize, end: usize) -> Self {
      IntersectionRef {
         first_px: first_px,
         start: start,
         end: end,
      }
   }
}

impl Default for IntersectionRef {
   #[inline]
   fn default() -> IntersectionRef {
      IntersectionRef::new(0, 0, 0)
   }
}


#[derive(Debug, Clone, Copy)]
pub struct Segment {
   pub p1: usize,
   pub p2: usize,
}

impl Segment {
   #[inline]
   pub fn new(p1: usize, p2: usize) -> Self {
      Segment {
         p1: p1,
         p2: p2,
      }
   }
}

impl Default for Segment {
   fn default() -> Segment {
      Segment::new(0, 0)
   }
}

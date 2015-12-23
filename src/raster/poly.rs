
#[derive(Debug, Clone)]
pub struct Poly {
   pub start: usize,
   pub end: usize,
   pub color: usize,
}

impl Poly {
   #[inline]
   pub fn new(start: usize, end: usize, color: usize) -> Self {
      Poly {
         start: start,
         end: end,
         color: color,
      }
   }
}

impl Default for Poly {
   fn default() -> Poly {
      Poly::new(0, 0, 0)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct PolyRef {
   pub start: usize,
   pub end: usize,
   pub src: usize,
}

impl PolyRef {
   #[inline]
   pub fn new(start: usize, end: usize, src: usize) -> Self {
      PolyRef {
         start: start,
         end: end,
         src: src,
      }
   }
}

impl Default for PolyRef {
   fn default() -> PolyRef {
      PolyRef::new(0, 0, 0)
   }
}

use super::Poly;

pub struct Scene {
   pub polys: Vec<Poly>,
}

impl Scene {
   #[inline]
   pub fn new() -> Self {
      Scene {
         polys: Vec::new(),
      }
   }

   #[inline]
   pub fn push(&mut self, poly: Poly) {
      self.polys.push(poly);
   }
}

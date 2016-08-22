use super::Poly;

pub struct Scene {
   pub polys: Vec<Box<Poly>>,
}

impl Scene {
   #[inline]
   pub fn new() -> Self {
      Scene {
         polys: Vec::new(),
      }
   }

   #[inline]
   pub fn push(&mut self, poly: Box<Poly>) {
      self.polys.push(poly);
   }
}

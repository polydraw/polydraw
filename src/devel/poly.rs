use data::IntPoint;
use draw::RGB;

#[derive(Debug, Clone)]
pub struct Poly {
   pub contours: Vec<Vec<IntPoint>>,
   pub color: RGB,
}

impl Poly {
   #[inline]
   pub fn new(contours: Vec<Vec<IntPoint>>, color: RGB) -> Self {
      Poly {
         contours: contours,
         color: color,
      }
   }
}

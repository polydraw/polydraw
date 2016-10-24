use geom::point::Point;
use draw::RGB;

#[derive(Debug, Clone)]
pub struct Poly {
   pub contours: Vec<Vec<Point>>,
   pub color: RGB,
}

impl Poly {
   #[inline]
   pub fn new(contours: Vec<Vec<Point>>, color: RGB) -> Self {
      Poly {
         contours: contours,
         color: color,
      }
   }
}

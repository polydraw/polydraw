use geom::point::Point;
use draw::RGB;

pub struct Poly {
   pub points: Vec<Point>,
   pub color: RGB,
}

impl Poly {
   #[inline]
   pub fn new(points: Vec<Point>, color: RGB) -> Self {
      Poly {
         points: points,
         color: color,
      }
   }
}

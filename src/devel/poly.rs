use geom::point::Point;
use draw::RGB;

pub struct Poly {
   pub points: Vec<Point>,
   pub holes: Vec<Vec<Point>>,
   pub color: RGB,
}

impl Poly {
   #[inline]
   pub fn new(points: Vec<Point>, color: RGB) -> Self {
      Poly {
         points: points,
         holes: vec![],
         color: color,
      }
   }

   #[inline]
   pub fn new_with_holes(
      points: Vec<Point>,
      holes: Vec<Vec<Point>>,
      color: RGB
   ) -> Self {
      Poly {
         points: points,
         holes: holes,
         color: color,
      }
   }
}

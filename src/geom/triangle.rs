use super::point::Point;

#[derive(Debug)]
pub struct Triangle {
   pub a: Point,
   pub b: Point,
   pub c: Point
}

impl Triangle {
   pub fn new(a: Point, b: Point, c: Point) -> Self {
      Triangle {
         a: a,
         b: b,
         c: c
      }
   }
}

use super::point::Point;

#[derive(Debug)]
pub struct Triangle {
   pub v0: Point,
   pub v1: Point,
   pub v2: Point
}

impl Triangle {
   pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
      Triangle {
         v0: v0,
         v1: v1,
         v2: v2
      }
   }
}

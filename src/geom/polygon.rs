use super::point::Point;

#[derive(Debug)]
pub struct Polygon {
   pub points: Vec<Point>
}

impl Polygon {
   pub fn new(points: Vec<Point>) -> Self {
      Polygon {
         points: points
      }
   }
}

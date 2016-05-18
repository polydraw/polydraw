extern crate polydraw;

use polydraw::geom::point::Point;


#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub p1: Point,
   pub p2: Point,
}


impl Edge {
   #[inline]
   pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      Edge {
         p1: Point::new(x1, y1),
         p2: Point::new(x2, y2),
      }
   }
}


fn main() {
   let edges = vec![
      Edge::new(0, 35, 15, 15),
      Edge::new(0, 32, 15, 23),
      Edge::new(0, 12, 15, 35),
      Edge::new(0, 12, 15, 0),
      Edge::new(0, 18, 15, 23),
      Edge::new(0, 0, 15, 20),
   ];

   for edge in edges {
      println!("{:?}", edge);
   }

}

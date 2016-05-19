extern crate polydraw;

use std::cmp::min;

use polydraw::geom::point::Point;
use polydraw::raster::create_default_vec;


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


fn calc_edges_min_y(edges: &Vec<Edge>, edges_min_y: &mut Vec<i64>) {
   for (index, edge) in edges.iter().enumerate() {
      let min_y = min(edge.p1.y, edge.p2.y);
      edges_min_y[index] = min_y;
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

   let mut edges_min_y: Vec<i64> = create_default_vec(edges.len());

   calc_edges_min_y(&edges, &mut edges_min_y);

   for (index, edge) in edges.iter().enumerate() {
      println!("{:?} - {}", edge, edges_min_y[index]);
   }
}


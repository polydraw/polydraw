extern crate polydraw;

use polydraw::geom::point::Point;
use polydraw::geom::ring::Ring;

fn h_intersect_line(p1: Point, p2: Point, step: i64, inters: &mut Ring<i64>)  {
}

fn main() {
   let mut inters = Ring::new(10000);
   let p1 = Point::new(2135, 2476);
   let p2 = Point::new(16753, 12398);
   h_intersect_line(p1, p2, 1000, &mut inters);
}


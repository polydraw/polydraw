extern crate polydraw;

use polydraw::geom::point::Point;
use polydraw::geom::ring::Ring;
use polydraw::geom::lineinter::{h_multi_intersect, h_multi_intersect_fast};

fn main() {
   let mut inters = Ring::new(100_000);
   let mut inters_fast = Ring::new(100_000);

   let p1 = Point::new(16_753, 2_476);
   let p2 = Point::new(2_135, 1_534_398);

   h_multi_intersect(p1, p2, 1000, &mut inters);
   h_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

   for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
      assert_eq!(correct, fast);
   }

   // for x in inters[..].iter() {
   //    println!("X : {}", x);
   // }
}

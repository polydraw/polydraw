extern crate polydraw;

use polydraw::geom::ring::Ring;
use polydraw::geom::clip2::{h_split, v_split, Edge, InclinedEdge, VerticalEdge, HorizontalEdge};


#[inline]
fn print_edges(name: &str, edges: &Ring<Edge>) {
   print!("{}: ", name);

   println!("s {} e {} :", edges.start(), edges.end());

   for edge in edges[..].iter() {
      println!("{:?}", edge);
   }
}


fn main() {
   let mut left = Ring::new(1024);
   let mut right = Ring::new(1024);

   right.push(Edge::Vertical(
      VerticalEdge::new(100_000, 100_000, 600_000)
   ));
   right.push(Edge::Inclined(
      InclinedEdge::new(100_000, 600_000, 300_000, 800_000)
   ));
   right.push(Edge::Horizontal(
      HorizontalEdge::new(300_000, 600_000, 800_000)
   ));
   right.push(Edge::Vertical(
      VerticalEdge::new(600_000, 800_000, 600_000)
   ));
   right.push(Edge::Inclined(
      InclinedEdge::new(600_000, 600_000, 100_000, 100_000)
   ));

   v_split(300_000, &mut left, &mut right);

   print_edges("LEFT", &left);
   print_edges("RIGHT", &right);
}

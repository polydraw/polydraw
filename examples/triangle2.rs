extern crate polydraw;

use polydraw::geom::ring::Ring;
use polydraw::geom::clip2::{h_split, Edge, InclinedEdge, VerticalEdge, HorizontalEdge};


#[inline]
fn print_edges(name: &str, edges: &Ring<Edge>) {
   print!("{}: ", name);

   println!("s {} e {} :", edges.start(), edges.end());

   for edge in edges[..].iter() {
      println!("{:?}", edge);
   }
}


fn main() {
   let mut lower = Ring::new(1024);
   let mut upper = Ring::new(1024);

   upper.push(Edge::Vertical(
      VerticalEdge::new(100_000, 100_000, 600_000)
   ));
   upper.push(Edge::Inclined(
      InclinedEdge::new(100_000, 600_000, 300_000, 800_000)
   ));
   upper.push(Edge::Horizontal(
      HorizontalEdge::new(300_000, 600_000, 800_000)
   ));
   upper.push(Edge::Vertical(
      VerticalEdge::new(600_000, 800_000, 600_000)
   ));
   upper.push(Edge::Inclined(
      InclinedEdge::new(600_000, 600_000, 100_000, 100_000)
   ));

   h_split(400_000, &mut lower, &mut upper);

   print_edges("LOWER", &lower);
   print_edges("UPPER", &upper);
}

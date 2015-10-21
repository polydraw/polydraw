extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;


struct TriangleRenderer {
   triangles: Vec<Point>,
}

impl TriangleRenderer {
   fn new() -> Self {
      TriangleRenderer {
         triangles: vec![],
      }
   }
}


impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();
   }
}


fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle3")
      .run();
}

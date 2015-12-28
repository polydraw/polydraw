extern crate polydraw;

use polydraw::geom::point::Point;
use polydraw::{Application, Renderer, Frame};
use polydraw::raster::{Scene, Rasterizer, EdgeType, Poly};


#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub p1: Point,
   pub p2: Point,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, p1: Point, p2: Point) -> Self {
      Edge {
         edge_type: edge_type,
         p1: p1,
         p2: p2,
      }
   }

   #[inline]
   pub fn vert_top(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LVT, p1, p2)
   }

   #[inline]
   pub fn vert_bottom(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LVB, p1, p2)
   }

   #[inline]
   pub fn hori_right(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LHR, p1, p2)
   }

   #[inline]
   pub fn hori_left(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LHL, p1, p2)
   }

   #[inline]
   pub fn top_right(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LTR, p1, p2)
   }

   #[inline]
   pub fn top_left(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LTL, p1, p2)
   }

   #[inline]
   pub fn bottom_right(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LBR, p1, p2)
   }

   #[inline]
   pub fn bottom_left(p1: Point, p2: Point) -> Edge {
      Edge::new(EdgeType::LBL, p1, p2)
   }
}


struct ClipRenderer {
   rasterizer: Rasterizer,
   div_per_pixel: i64,

   edges: Vec<Edge>,
   polys: Vec<Poly>,
}

impl ClipRenderer {
   fn new() -> Self {
      let edges = vec![
         Edge::top_left(Point::new(5, 1), Point::new(1, 6)),       // 0
         Edge::top_right(Point::new(1, 6), Point::new(8, 9)),      // 1
         Edge::bottom_right(Point::new(8, 9), Point::new(9, 6)),   // 2
         Edge::bottom_left(Point::new(9, 6), Point::new(5, 1)),    // 3
         Edge::top_right(Point::new(1, 1), Point::new(3, 8)),      // 4
         Edge::bottom_right(Point::new(3, 8), Point::new(10, 4)),  // 5
         Edge::bottom_left(Point::new(10, 4), Point::new(1, 1)),   // 6
      ];

      let polys = vec![
         Poly::new(0, 4, 0),    // 0
         Poly::new(4, 7, 1),    // 1
      ];

      ClipRenderer {
         rasterizer: Rasterizer::new(),
         div_per_pixel: 1000,

         edges: edges,
         polys: polys,
      }
   }

   fn create_scene(&self) -> Scene {
      let points = vec![];

      let segments = vec![];

      let circles = vec![];

      let edges = vec![];

      let polys = vec![];

      let colors = vec![];

      Scene {
         points: points,
         segments: segments,
         circles: circles,
         edges: edges,
         polys: polys,
         colors: colors,
      }
   }
}

impl Renderer for ClipRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let scene = self.create_scene();

      self.rasterizer.render(&scene, frame, self.div_per_pixel);
   }
}

fn main() {
   let mut renderer = ClipRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Clip")
      .run();
}

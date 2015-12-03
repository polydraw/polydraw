extern crate polydraw;

use std::usize;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::raster::{Scene, Point, Segment, Circle, EdgeType, EdgeSrc, Poly, Rasterizer};

struct SceneRenderer {
   rasterizer: Rasterizer,
}

impl SceneRenderer {
   fn new() -> Self {
      SceneRenderer {
         rasterizer: Rasterizer::new(),
      }
   }

   fn create_scene(&self) -> Scene {
      let points = vec![
         Point::new(0, 0),   // 0
         Point::new(8, 0),   // 1
         Point::new(12, 0),  // 2
         Point::new(12, 4),  // 3
         Point::new(0, 8),   // 4
         Point::new(0, 12),  // 5
         Point::new(4, 12),  // 6
         Point::new(12, 12), // 7
         Point::new(8, 4),   // 8
         Point::new(4, 8),   // 9
      ];

      let segments = vec![
         Segment::new(0, 1), // a 0
         Segment::new(1, 2), // b 1
         Segment::new(0, 4), // c 2
         Segment::new(1, 3), // d 3
         Segment::new(2, 3), // e 4
         Segment::new(3, 7), // f 5
         Segment::new(4, 5), // g 6
         Segment::new(4, 6), // h 7
         Segment::new(5, 6), // i 8
         Segment::new(6, 7), // j 9
      ];

      let circles = vec![
         Circle::new(8, 4),  // 0
         Circle::new(9, 4),  // 1
      ];

      let edges = vec![
         EdgeSrc::new(EdgeType::LVT, 2, usize::MAX),
         EdgeSrc::new(EdgeType::CTR, 7, 1),
         EdgeSrc::new(EdgeType::LHR, 9, usize::MAX),
         EdgeSrc::new(EdgeType::LVB, 5, usize::MAX),
         EdgeSrc::new(EdgeType::CBL, 3, 0),
         EdgeSrc::new(EdgeType::LHL, 0, usize::MAX),
         EdgeSrc::new(EdgeType::ATR, 3, 0),
         EdgeSrc::new(EdgeType::LVB, 4, usize::MAX),
         EdgeSrc::new(EdgeType::LHL, 1, usize::MAX),
         EdgeSrc::new(EdgeType::LVT, 6, usize::MAX),
         EdgeSrc::new(EdgeType::LHR, 8, usize::MAX),
         EdgeSrc::new(EdgeType::ABL, 7, 1),
      ];

      let polys = vec![
         Poly::new(0, 6, 0),
         Poly::new(6, 9, 1),
         Poly::new(9, 12, 2),
      ];

      let colors = vec![
         RGB::new(194, 243, 137),
         RGB::new(154, 222, 76),
         RGB::new(172, 58, 162),
      ];

      Scene {
         points: points,
         segments: segments,
         circles: circles,
         edges: edges,
         polys: polys,
         colors: colors,
      }
   }

   fn scale_scene(&self, scene: &mut Scene) {
      let scale = 50_000;

      for point in &mut scene.points {
         point.x = scale * point.x;
         point.y = scale * point.y;
      }

      for circle in &mut scene.circles {
         circle.radius = scale * circle.radius;
      }
   }
}

impl Renderer for SceneRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      let mut scene = self.create_scene();
      self.scale_scene(&mut scene);

      self.rasterizer.render(&scene, frame);
   }
}

fn main() {
   let mut renderer = SceneRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Scene")
      .run();
}

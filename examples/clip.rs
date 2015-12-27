extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::raster::{Scene, Rasterizer};

struct ClipRenderer {
   rasterizer: Rasterizer,
   div_per_pixel: i64,
}

impl ClipRenderer {
   fn new() -> Self {
      ClipRenderer {
         rasterizer: Rasterizer::new(),
         div_per_pixel: 1000,
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

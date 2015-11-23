extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::raster::{Scene, Point, Segment, Circle, EdgeType, Edge, Poly};
use polydraw::geom::number::NumberOps;

const DIV_PER_PIXEL: i64 = 1000;

#[inline]
fn to_px(v: i64) -> i64 {
   v / DIV_PER_PIXEL
}

#[inline]
fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

struct SceneRenderer {
   index: usize,
}

impl SceneRenderer {
   fn new() -> Self {
      SceneRenderer {
         index: 0
      }
   }
}

impl Renderer for SceneRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();
   }
}

fn main() {
   let mut renderer = SceneRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Scene")
      .run();
}

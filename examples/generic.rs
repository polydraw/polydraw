extern crate polydraw;

mod support;

use polydraw::{Application, Renderer, RenderFrame};

use support::update_data;

struct NoiseRenderer {
   counter: u64,
   seed: u64,
}

impl NoiseRenderer {
   fn new() -> Self {
      NoiseRenderer {
         counter: 0,
         seed: 0,
      }
   }
}

impl Renderer for NoiseRenderer {
   fn render(&mut self, render_frame: &mut RenderFrame) {
      self.counter += 1;
      self.seed = self.counter;
      update_data(&mut render_frame.data, render_frame.width, render_frame.height, &mut self.seed);
   }
}

fn main() {
   let mut renderer = NoiseRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Generic")
      .size(800, 400)
      .position(400, 200)
      .run();
}

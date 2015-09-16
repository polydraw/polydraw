extern crate polydraw;

use polydraw::{Application, Renderer, RenderFrame};

pub fn rand_u8(seed: &mut u64) -> u8 {
    *seed = seed.wrapping_mul(58321).wrapping_add(11113);
    (seed.wrapping_shr(16) % 256) as u8
}

pub fn update_data(data: &mut Vec<u8>, width: u32, height: u32, seed: &mut u64) {
   for y in 0..height {
      for x in 0..width {
         let i: usize = (3 * (x + y * width)) as usize;
         let r = rand_u8(seed);
         data[i] = r;
         data[i + 1] = r;
         data[i + 2] = r;
      }
   }
}

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
      .title("Noise")
      .size(800, 400)
      .position(400, 200)
      .run();
}

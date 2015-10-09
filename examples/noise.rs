extern crate polydraw;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;

pub fn rand_u8(seed: &mut u64) -> u8 {
    *seed = seed.wrapping_mul(58321).wrapping_add(11113);
    (seed.wrapping_shr(16) % 256) as u8
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
   fn render(&mut self, frame: &mut Frame) {
      self.counter += 1;
      self.seed = self.counter;

      for y in 0..frame.height as i32 {
         for x in 0..frame.width as i32 {
            let r = rand_u8(&mut self.seed);
            frame.put_pixel(x, y, &RGB::new(r, r, r));
         }
      }
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

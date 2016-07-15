use std::iter::repeat;

use error::{RuntimeError, VoidResult};
use frame::GPUFrame;
use draw::RGB;

use super::super::{clear, draw_pixels};


pub struct DrawPixelsFrame {
   pub data: Vec<u8>,
}

impl GPUFrame for DrawPixelsFrame {
   #[inline]
   fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let data = Self::create_data(width, height);

      Ok(DrawPixelsFrame {
         data: data,
      })
   }

   #[inline]
   fn clear(&mut self) {
      for item in self.data.iter_mut() {
         *item = 0;
      }
   }

   #[inline]
   fn put_pixel(&mut self, x: i32, y: i32, color: &RGB, width: u32, height: u32) {
      if x >= width as i32 || y >= height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 4 * (x + y * width as i32) as usize;
      self.data[i    ] = color.r;
      self.data[i + 1] = color.g;
      self.data[i + 2] = color.b;
   }

   #[inline]
   fn resize(&mut self, width: u32, height: u32) -> VoidResult {
      self.data.resize((width * height * 4) as usize, 0);

      Ok(())
   }

   #[inline]
   fn pre_render(&mut self) -> VoidResult {
      Ok(())
   }

   #[inline]
   fn post_render(&mut self, width: u32, height: u32) -> VoidResult {
      try!(clear());

      try!(draw_pixels(width, height, &self.data));

      Ok(())

   }
}

impl DrawPixelsFrame {
   #[inline]
   pub fn create_data(width: u32, height: u32) -> Vec<u8> {
      repeat(0u8)
         .take((width * height * 4) as usize)
         .collect::<Vec<_>>()
   }
}

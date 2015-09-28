use std::iter::repeat;

use super::draw::RGB;

pub struct RenderFrame {
   pub width: u32,
   pub height: u32,
   pub max_width: u32,
   pub max_height: u32,
   pub data: Vec<u8>,
}

impl RenderFrame {
   pub fn new(width: u32, height: u32, screen_width: u32, screen_height: u32) -> Self {
      let data = Self::create_data(screen_width, screen_height);

      RenderFrame {
         width: width,
         height: height,
         max_width: screen_width,
         max_height: screen_height,
         data: data,
      }
   }

   #[inline]
   pub fn create_data(width: u32, height: u32) -> Vec<u8> {
      repeat(0u8)
         .take((width * height * 3) as usize)
         .collect::<Vec<_>>()
   }

   pub fn clear(&mut self) {
      for i in 0 as usize..(self.width * self.height * 3) as usize {
         self.data[i] = 0;
      }
   }

   pub fn put_pixel(&mut self, x: i32, y: i32, color: &RGB) {
      if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 3 * (x + y * self.width as i32) as usize;
      self.data[i    ] = color.r;
      self.data[i + 1] = color.g;
      self.data[i + 2] = color.b;
   }
}

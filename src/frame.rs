use std::iter::repeat;

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
}

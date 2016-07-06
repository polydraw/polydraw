use std::ptr;

use error::{RuntimeError, VoidResult};
use frame::GPUFrame;
use draw::RGB;

use super::{Texture, Framebuffer, Buffer};


pub struct GLFrame {
   pub texture: Texture,
   pub framebuffer: Framebuffer,
   pub buffer: Buffer,
}

impl GPUFrame for GLFrame {
   #[inline]
   fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let texture = try!(Texture::new(width, height));
      let framebuffer = try!(Framebuffer::new(&texture));
      let mut buffer = try!(Buffer::new());

      try!(texture.bind());
      try!(framebuffer.bind());

      try!(buffer.bind());
      try!(buffer.init_data((width * height * 4) as usize));

      Ok(GLFrame {
         texture: texture,
         framebuffer: framebuffer,
         buffer: buffer,
      })
   }

   #[inline]
   fn clear(&mut self) {
      unsafe {
         ptr::write_bytes(
            self.buffer.ptr as *mut u8, 0, self.buffer.size as usize
         )
      };
   }

   #[inline]
   fn put_pixel(&mut self, x: i32, y: i32, color: &RGB, width: u32, height: u32) {
      if x >= width as i32 || y >= height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 4 * (x + y * width as i32) as isize;
      let p = self.buffer.ptr as *mut u8;
      unsafe {
         *p.offset(i) = color.r;
         *p.offset(i+1) = color.g;
         *p.offset(i+2) = color.b;
      }
   }

   #[inline]
   fn resize(&mut self, width: u32, height: u32) -> VoidResult {
      try!(self.buffer.init_data((width * height * 4) as usize));

      self.texture.resize(width, height)
   }

   #[inline]
   fn pre_render(&mut self) -> VoidResult {
      self.buffer.map()
   }

   #[inline]
   fn post_render(&mut self, width: u32, height: u32) -> VoidResult {
      try!(self.buffer.unmap());

      try!(self.texture.update(width, height));

      self.framebuffer.blit(width, height)
   }
}

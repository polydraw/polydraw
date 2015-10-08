use error::RuntimeError;

use sys::gl;

pub struct GlContext {
   pub texture: gl::Texture,
   pub framebuffer: gl::Framebuffer,
   pub buffer: gl::Buffer,
}

impl GlContext {
   pub fn new(width: u32, height: u32, screen_width: u32, screen_height: u32) -> Result<Self, RuntimeError> {
      let texture = gl::Texture::new(width, height);

      let framebuffer = gl::Framebuffer::new(&texture);

      let buffer = gl::Buffer::new(width, height, screen_width, screen_height);

      Ok(GlContext {
         texture: texture,
         framebuffer: framebuffer,
         buffer: buffer,
      })
   }
}

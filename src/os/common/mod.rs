use error::RuntimeError;

use sys::gl;

pub struct GlInitializer {
   pub texture: gl::Texture,
   pub framebuffer: gl::Framebuffer,
}

impl GlInitializer {
   pub fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let texture = gl::Texture::new(width, height);

      let framebuffer = gl::Framebuffer::new(&texture);

      Ok(GlInitializer {
         texture: texture,
         framebuffer: framebuffer,
      })
   }
}

use error::RuntimeError;

use sys::gl;

pub struct GlContext {
   pub texture: gl::Texture,
   pub framebuffer: gl::Framebuffer,
}

impl GlContext {
   pub fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let texture = gl::Texture::new(width, height);

      let framebuffer = gl::Framebuffer::new(&texture);

      Ok(GlContext {
         texture: texture,
         framebuffer: framebuffer,
      })
   }
}

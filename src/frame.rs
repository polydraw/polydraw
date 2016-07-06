use error::{RuntimeError, VoidResult};
use draw::RGB;
use renderer::Renderer;


pub trait GPUFrame {
   fn new(
      width: u32, height: u32
   ) -> Result<Self, RuntimeError> where Self: Sized;

   fn clear(&mut self);

   fn put_pixel(&mut self, x: i32, y: i32, color: &RGB, width: u32, height: u32);

   fn resize(&mut self, width: u32, height: u32) -> VoidResult;

   fn pre_render(&mut self) -> VoidResult;

   fn post_render(&mut self, width: u32, height: u32) -> VoidResult;
}

pub struct Frame {
   pub width: u32,
   pub height: u32,
   gpu_frame: Box<GPUFrame>,
}

impl Frame {
   #[inline]
   pub fn new(
      width: u32,
      height: u32,
      gpu_frame: Box<GPUFrame>
   ) -> Result<Self, RuntimeError> {
      Ok(Frame {
         width: width,
         height: height,
         gpu_frame: gpu_frame,
      })
   }

   #[inline]
   pub fn clear(&mut self) {
      self.gpu_frame.clear();
   }

   #[inline]
   pub fn put_pixel(&mut self, x: i32, y: i32, color: &RGB) {
      self.gpu_frame.put_pixel(x, y, color, self.width, self.height);
   }

   #[inline]
   pub fn resize(&mut self, width: u32, height: u32) -> VoidResult {
      self.width = width;
      self.height = height;

      self.gpu_frame.resize(width, height)
   }

   #[inline]
   pub fn render(&mut self, renderer: &mut Renderer) -> VoidResult {
      try!(self.gpu_frame.pre_render());

      renderer.render(self);

      self.gpu_frame.post_render(self.width, self.height)
   }
}


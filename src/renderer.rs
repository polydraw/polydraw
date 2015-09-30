use super::frame::RenderFrame;

pub trait Renderer {
   fn render(&mut self, &mut RenderFrame);

   #[allow(unused_variables)]
   fn init(&mut self, &RenderFrame) {}

   #[allow(unused_variables)]
   fn mouse_moved(&mut self, x: i32, y: i32) {}

   #[allow(unused_variables)]
   fn resized(&mut self, width: u32, height: u32) {}
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
   fn render(&mut self, _: &mut RenderFrame) {}
}

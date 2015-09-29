use super::frame::RenderFrame;

pub trait Renderer {
   #[allow(unused_variables)]
   fn init(&mut self, &RenderFrame) {}

   fn render(&mut self, &mut RenderFrame);

   #[allow(unused_variables)]
   fn mouse_moved(&mut self, x: i32, y: i32) {}
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
   fn render(&mut self, _: &mut RenderFrame) {}
}

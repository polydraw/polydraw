use super::frame::RenderFrame;

pub trait Renderer {
   fn render(&mut self, &mut RenderFrame);

   fn mouse_moved(&mut self, x: i32, y: i32) {}
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
   fn render(&mut self, _: &mut RenderFrame) {}
}

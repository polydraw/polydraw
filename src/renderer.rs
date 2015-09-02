use super::frame::RenderFrame;

pub trait Renderer {
   fn render(&mut self, &mut RenderFrame);
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
   fn render(&mut self, _: &mut RenderFrame) {}
}

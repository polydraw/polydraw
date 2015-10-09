use frame::Frame;

#[allow(unused_variables)]
pub trait Renderer {
   fn render(&mut self, &mut Frame);

   fn init(&mut self, width: u32, height: u32) {}

   fn resized(&mut self, width: u32, height: u32) {}

   fn mouse_moved(&mut self, x: i32, y: i32) {}

   fn mouse_left_button_pressed(&mut self) {}
   fn mouse_left_button_released(&mut self) {}

   fn mouse_middle_button_pressed(&mut self) {}
   fn mouse_middle_button_released(&mut self) {}

   fn mouse_right_button_pressed(&mut self) {}
   fn mouse_right_button_released(&mut self) {}

   fn mouse_extra_button_pressed(&mut self, n: u8) {}
   fn mouse_extra_button_released(&mut self, n: u8) {}
}

pub struct NullRenderer;

impl Renderer for NullRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();
   }
}

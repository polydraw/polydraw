#[derive(Clone, Debug)]
pub enum Event {
   Quit,

   Resized(u32, u32),

   MouseMoved(i32, i32),

   MouseLeftButtonPressed,
   MouseLeftButtonReleased,

   MouseMiddleButtonPressed,
   MouseMiddleButtonReleased,

   MouseRightButtonPressed,
   MouseRightButtonReleased,

   MouseExtraButtonPressed(u8),
   MouseExtraButtonReleased(u8),
}

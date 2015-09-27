#[derive(Clone, Debug)]
pub enum Event {
   Quit,

   Resized(u32, u32),

   MouseMoved(i32, i32),
}

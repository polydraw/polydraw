
pub struct HookFn<Args, Out> {
   pub func: Box<FnMut(Args) -> Out>,
}

impl<Args, Out> HookFn<Args, Out> {
   pub fn new<F>(func: F) -> Self where F: FnMut(Args) -> Out + 'static {
      HookFn {
         func: Box::new(func)
      }
   }

   pub fn reassign<F>(&mut self, func: F) where F: FnMut(Args) -> Out + 'static {
      self.func = Box::new(func);
   }
}

pub struct Hooks {
   pub first: HookFn<u32, ()>,
}

impl Hooks {
   pub fn initialize() -> Self {
      Hooks {
         first: HookFn::new(first_default)
      }
   }
}

pub fn first_default(x: u32) {
   println!("First Default - {}", x);
}

pub fn second(x: u32) {
   println!("Second - {}", x);
}

/*
fn main() {
   let mut hooks = Hooks::initialize();
   (hooks.first.func)(87);

   hooks.first.reassign(second);

   (hooks.first.func)(100);
}
*/

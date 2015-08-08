extern crate polydraw;

use polydraw::os::win;
use polydraw::os::wgl;
use polydraw::os::gl;

fn main() {
   let width: usize = 1280;
   let height: usize = 720;

   let window = win::Window::create(width, height, "Win Min", "WinMinClass");
   window.show_normal();

   let dc = window.dc();
   println!("DC: {:?}", dc);

   match wgl::init_pixel_format(dc) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   let context = match wgl::Context::create(dc) {
      Ok(context) => context,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("RC: {:?}", context.rc);

   match wgl::Context::current() {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   gl::load(wgl::Loader::new());

   loop {
      let message = match win::Message::get() {
         Some(message) => message,
         None => break
      };

      message.translate();
      message.dispatch();

      gl::clear_color(0.0, 0.7, 1.0, 1.0);
      gl::clear();
      gl::flush();

      wgl::swap_buffers(dc);
   }
}

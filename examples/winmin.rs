extern crate polydraw;

mod common;

use polydraw::os::win;
use polydraw::os::wgl;
use polydraw::os::gl;

use common::{rand_u8, create_data, update_data};

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

   gl::load(wgl::Loader::new());

   let mut counter: u64 = 0;
   let mut data = create_data(width, height);

   let texture = gl::Texture::new(width, height);

   println!("GL texture ................ : {:?}", texture.name);

   let framebuffer = gl::Framebuffer::new(&texture);

   println!("GL framebuffer ............ : {:?}", framebuffer.name);

   let mut seed = 0;

   loop {
      let message = match win::Message::get() {
         Some(message) => message,
         None => break
      };

      message.translate();
      message.dispatch();

      counter += 1;
      seed = counter;

      update_data(&mut data, width, height, &mut seed);

      texture.update(width, height, &data);

      framebuffer.blit(width, height);

      gl::flush();

      wgl::swap_buffers(dc);
   }
}

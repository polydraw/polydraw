use error::RuntimeError;
use application::OsApplication;
use renderer::Renderer;
use event::Event;
use os::common::GlContext;

pub struct EventLoop<'a> {
   pub os_application: &'a OsApplication,
}

impl<'a> EventLoop<'a> {
   pub fn new(os_application: &'a OsApplication) -> Self {
      EventLoop {
         os_application: os_application,
      }
   }

   pub fn run(
      &self, renderer: &mut Renderer, width: u32, height: u32, screen_width: u32, screen_height: u32
   ) -> Result<(), RuntimeError> {
      let mut gl = try!(GlContext::new(width, height, screen_width, screen_height));

      let texture = &gl.texture;
      let framebuffer = &gl.framebuffer;
      let buffer = &mut gl.buffer;

      texture.bind();
      framebuffer.bind();

      buffer.bind();
      buffer.data();
      buffer.unbind();

      renderer.init(buffer.width, buffer.height);

      let mut quit = false;

      loop {
         let mut new_width = buffer.width;
         let mut new_height = buffer.height;

         for event in self.os_application.poll_events() {
            match event {
               Event::Resized(width, height) => {
                  new_width = width;
                  new_height = height;
               },

               Event::MouseMoved(x, y) => {
                  renderer.mouse_moved(x, new_height as i32 - y - 1);
               },

               Event::MouseLeftButtonPressed => {
                  renderer.mouse_left_button_pressed();
               },

               Event::MouseLeftButtonReleased => {
                  renderer.mouse_left_button_released();
               },

               Event::MouseMiddleButtonPressed => {
                  renderer.mouse_middle_button_pressed();
               },

               Event::MouseMiddleButtonReleased => {
                  renderer.mouse_middle_button_released();
               },

               Event::MouseRightButtonPressed => {
                  renderer.mouse_right_button_pressed();
               },

               Event::MouseRightButtonReleased => {
                  renderer.mouse_right_button_released();
               },

               Event::MouseExtraButtonPressed(n) => {
                  renderer.mouse_extra_button_pressed(n);
               },

               Event::MouseExtraButtonReleased(n) => {
                  renderer.mouse_extra_button_released(n);
               },

               Event::Quit => {
                  quit = true;
                  break
               }
            }
         }

         if quit {
            break
         }

         if new_width != buffer.width || new_height != buffer.height {
            buffer.resize(new_width, new_height);
            texture.resize(new_width, new_height);
            renderer.resized(new_width, new_height);
         }

         buffer.bind();
         buffer.map();

         renderer.render(buffer);

         buffer.unmap();

         buffer.unbind();

         texture.update(buffer.width, buffer.height, &buffer);

         framebuffer.blit(buffer.width, buffer.height);

         try!(self.os_application.swap_buffers());
      }

      Ok(())
   }
}

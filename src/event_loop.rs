use error::RuntimeError;
use application::OsApplication;
use frame::RenderFrame;
use renderer::Renderer;
use event::Event;

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
      &self, renderer: &mut Renderer, render_frame: &mut RenderFrame
   ) -> Result<(), RuntimeError> {
      let texture = &self.os_application.gl.texture;
      let framebuffer = &self.os_application.gl.framebuffer;

      framebuffer.bind();

      renderer.init(render_frame.width, render_frame.height);

      let mut quit = false;

      loop {
         let current_width = render_frame.width;
         let current_height = render_frame.height;

         for event in self.os_application.poll_events() {
            match event {
               Event::Resized(width, height) => {
                  render_frame.width = width;
                  render_frame.height = height;
               },

               Event::MouseMoved(x, y) => {
                  renderer.mouse_moved(x, render_frame.height as i32 - y - 1);
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

         if current_width != render_frame.width || current_height != render_frame.height {
            texture.resize(render_frame.width, render_frame.height);
            renderer.resized(render_frame.width, render_frame.height);
         }

         renderer.render(render_frame);

         texture.update(render_frame.width, render_frame.height, &render_frame.data);

         framebuffer.blit(render_frame.width, render_frame.height);

         try!(self.os_application.swap_buffers());
      }

      Ok(())
   }
}

use error::VoidResult;
use application::OsApplication;
use renderer::Renderer;
use frame::Frame;
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

   pub fn run(&self, renderer: &mut Renderer, width: u32, height: u32) -> VoidResult {
      renderer.init(width, height);

      let gpu_frame = try!(self.os_application.create_gpu_frame(width, height));

      let mut frame = try!(Frame::new(width, height, gpu_frame));

      let mut quit = false;

      loop {
         let mut new_width = frame.width;
         let mut new_height = frame.height;

         for event in self.os_application.poll_events() {
            match event {
               Event::Resized(width, height) => {
                  new_width = width;
                  new_height = height;
               },

               Event::MouseMoved(x, y) => {
                  renderer.mouse_moved(x, new_height as i32 - y - 1);
               },

               Event::Quit => {
                  quit = true;
                  break
               },

               _ => {
                  self.match_more_events(event, renderer)
               }
            }
         }

         if quit {
            break
         }

         if new_width != frame.width || new_height != frame.height {
            try!(frame.resize(new_width, new_height));
            renderer.resized(new_width, new_height);
         }

         try!(frame.render(renderer));

         try!(self.os_application.swap_buffers());
      }

      Ok(())
   }

   #[inline]
   fn match_more_events(&self, event: Event, renderer: &mut Renderer) {
      match event {
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

         _ => {
            panic!("Unhandled events");
         }
      }
   }
}

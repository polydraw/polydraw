use std::rc::Rc;

use error::RuntimeError;

use sys::xcb;
use sys::gl;
use sys::egl;

use frame::RenderFrame;
use super::initializer::XcbAtoms;


pub struct LinuxEventLoop<'a> {
   render_frame: &'a mut RenderFrame,
   connection: Rc<xcb::Connection>,
   window: &'a xcb::Window,
   atoms: &'a XcbAtoms,
   display: &'a egl::Display,
   surface: &'a egl::Surface,
   texture: &'a gl::Texture,
   framebuffer: &'a gl::Framebuffer,
}

impl<'a> LinuxEventLoop<'a> {
   pub fn new(
      render_frame: &'a mut RenderFrame,
      connection: &Rc<xcb::Connection>,
      window: &'a xcb::Window,
      atoms: &'a XcbAtoms,
      texture: &'a gl::Texture,
      framebuffer: &'a gl::Framebuffer,
      display: &'a egl::Display,
      surface: &'a egl::Surface,
   ) -> Self {
      LinuxEventLoop {
         render_frame: render_frame,
         connection: connection.clone(),
         window: window,
         atoms: atoms,
         display: display,
         surface: surface,
         texture: texture,
         framebuffer: framebuffer,
      }
   }

   pub fn run(&mut self) -> Result<(), RuntimeError> {
      let mut exit = false;

      let mut new_width: u32 = self.render_frame.width;
      let mut new_height: u32 = self.render_frame.height;

      loop {
         for result in self.connection.poll_event_iter() {
            let event = try!(result);

            let event_type = event.event_type();

            match event_type {
               xcb::EventType::Expose | xcb::EventType::Empty => {
                  if new_width != self.render_frame.width || new_height != self.render_frame.height {
                     self.render_frame.width = new_width;
                     self.render_frame.height = new_height;

                     self.texture.resize(self.render_frame.width, self.render_frame.height);
                  }

                  //update_data(&mut data, width, height, &mut seed);

                  self.texture.update(self.render_frame.width, self.render_frame.height, &self.render_frame.data);

                  self.framebuffer.blit(self.render_frame.width, self.render_frame.height);

                  gl::flush();

                  try!(egl::swap_buffers(&self.display, &self.surface));
               },
               xcb::EventType::ClientMessage => {
                  if event.is_close_event(&self.atoms.protocols_atom, &self.atoms.delete_window_atom) {
                     exit = true;
                     break;
                  }
               },
               xcb::EventType::ConfigureNotify => {
                  let (window_id, resize_width, resize_height) = event.resize_properties();

                  if window_id != self.window.window_id {
                     continue;
                  }

                  if (resize_width != self.render_frame.width) || (resize_height != self.render_frame.height) {
                     new_width = resize_width;
                     new_height = resize_height;
                  }
               },
               xcb::EventType::KeyPress => {
                  exit = true;
                  break;
               },
               _ => {}
            }
         }

         if exit {
            break;
         }
      }

      Ok(())
   }
}

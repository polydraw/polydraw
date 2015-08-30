use sys::xcb;

use super::application::EglSurfaceHandler;

pub struct LinuxWindow {
   pub window: xcb::Window,
   pub surface: EglSurfaceHandler,
   pub protocols_atom: xcb::Atom,
   pub delete_window_atom: xcb::Atom,
}

impl LinuxWindow {
   #[allow(unused_variables)]
   pub fn new(window: xcb::Window, surface: EglSurfaceHandler, title: &str) -> Self {
      window.set_title(title);

      window.map();

      let (protocols_atom, delete_window_atom) = window.register_close_event();

      LinuxWindow {
         window: window,
         surface: surface,
         protocols_atom: protocols_atom,
         delete_window_atom: delete_window_atom
      }
   }
}

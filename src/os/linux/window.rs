use sys::xcb;
use sys::egl;
use sys::gl;

pub struct LinuxWindow {
   pub window: xcb::Window,
   pub surface: egl::Surface,
   pub texture: gl::Texture,
   pub framebuffer: gl::Framebuffer,
   pub protocols_atom: xcb::Atom,
   pub delete_window_atom: xcb::Atom,
}

impl LinuxWindow {
   #[allow(unused_variables)]
   pub fn new(
      window: xcb::Window,
      surface: egl::Surface,
      texture: gl::Texture,
      framebuffer: gl::Framebuffer,
      title: &str
   ) -> Self {

      window.set_title(title);

      window.map();

      let (protocols_atom, delete_window_atom) = window.register_close_event();

      LinuxWindow {
         window: window,
         surface: surface,
         texture: texture,
         framebuffer: framebuffer,
         protocols_atom: protocols_atom,
         delete_window_atom: delete_window_atom
      }
   }
}

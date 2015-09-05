use error::RuntimeError;

use sys::win32;
use sys::wgl;
use sys::gl;

use super::desktop::WindowsDesktop;
use super::super::common::GlInitializer;

pub struct WindowsInitializer {
   pub window: win32::Window,
   pub device_context: win32::DeviceContext,
   pub wgl: WglInitializer,
   pub gl: GlInitializer,
}

impl WindowsInitializer {
   #[allow(unused_variables)]
   pub fn new(
      desktop: &WindowsDesktop, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(Self::init_window(title, width, height));

      let device_context = window.device_context();

      let wgl = try!(WglInitializer::new(&device_context));

      let gl = try!(GlInitializer::new(width, height));

      Ok(WindowsInitializer {
         window: window,
         device_context: device_context,
         wgl: wgl,
         gl: gl,
      })

      }

   #[inline]
   pub fn init_window(
      title: &str, width: u32, height: u32
   ) -> Result<win32::Window, RuntimeError> {
      let window = win32::Window::new(width, height, title, "PolyDrawWndClass");

      window.show_normal();

      Ok(window)
   }
}

pub struct WglInitializer {
   pub context: wgl::Context,
}

impl WglInitializer {
   pub fn new(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      try!(wgl::init_pixel_format(device_context));

      let context = try!(wgl::Context::create(device_context));

      Self::init_gl();

      Ok(WglInitializer {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl() {
      gl::load(wgl::Loader::new());
      gl::reset_pixelstore_alignment();
   }
}

use error::RuntimeError;

use sys::win32;
use sys::wgl;
use sys::gl;

use super::display::WindowsDisplay;
use super::super::common::GlContext;

pub struct WindowsWindow {
   pub window: win32::Window,
   pub device_context: win32::DeviceContext,
   pub wgl: WglContext,
   pub gl: GlContext,
}

impl WindowsWindow {
   pub fn new(
      _: &WindowsDisplay, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(Self::init_window(title, x, y, width, height));

      let device_context = window.device_context();

      let wgl = try!(WglContext::new(&device_context));

      let gl = try!(GlContext::new(width, height));

      Ok(WindowsWindow {
         window: window,
         device_context: device_context,
         wgl: wgl,
         gl: gl,
      })

      }

   #[inline]
   pub fn init_window(
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<win32::Window, RuntimeError> {
      let window = win32::Window::new(width, height, title, "PolyDrawWndClass");

      window.show_normal();

      window.position(x, y);

      Ok(window)
   }
}

pub struct WglContext {
   pub context: wgl::Context,
}

impl WglContext {
   pub fn new(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      try!(wgl::init_pixel_format(device_context));

      let context = try!(wgl::Context::create(device_context));

      Self::init_gl();

      Ok(WglContext {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl() {
      gl::load(wgl::Loader::new());
      gl::reset_pixelstore_alignment();
   }
}

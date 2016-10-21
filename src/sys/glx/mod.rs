#![cfg(target_os = "linux")]

pub mod ffi;

use libc::{
   c_int, c_void,
};

use std::mem;
use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind, VoidResult};

use super::xcb::ffi::xcb_window_t;
use super::x11::ffi::Display as X11Display;
use super::x11::ffi::XVisualInfo;
use super::x11;

use super::utils::fn_ptr::{FnPtrLoader, FnPtr};
use super::DynLibrary;

pub use self::ffi::GLXWindow;

pub type GLXNativeWindowType = xcb_window_t;

pub struct Loader;

impl Loader {
   pub fn new() -> Self {
      Loader
   }
}

impl FnPtrLoader for Loader {
   fn load(&self, name: &str) -> FnPtr {
      let cname = CString::new(name).unwrap();

      let addr = unsafe {
         ffi::glXGetProcAddress(cname.as_ptr())
      };

      addr
   }
}

pub struct Display {
   pub ptr: *mut X11Display
}

pub struct Config {
   pub ptr: ffi::GLXFBConfig
}

pub struct Context {
   pub ptr: ffi::GLXContext
}

pub struct Version {
   pub major: c_int,
   pub minor: c_int,
}

#[inline]
pub fn load_functions(library: &DynLibrary) -> VoidResult {
   unsafe {
      ffi::load_functions(library)
   };

   Ok(())
}

pub fn initialize(display: &Display) -> Result<Version, RuntimeError> {
   let mut major: c_int = unsafe {
      mem::uninitialized()
   };
   let mut minor: c_int = unsafe {
      mem::uninitialized()
   };

   let result = unsafe {
      ffi::glXQueryVersion(display.ptr, &mut major, &mut minor)
   };

   if result == 0 {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "glXQueryVersion failed".to_string()
      ));
   }

   Ok(Version {
      major: major,
      minor: minor
   })
}

pub fn choose_config(display: &Display, screen_id: &x11::ScreenID) -> Result<Config, RuntimeError> {
   let config_attribs = [
      ffi::GLX_X_RENDERABLE,         1,
      ffi::GLX_X_VISUAL_TYPE,        ffi::GLX_TRUE_COLOR,
      ffi::GLX_DRAWABLE_TYPE,        ffi::GLX_WINDOW_BIT,

      ffi::GLX_RENDER_TYPE,          ffi::GLX_RGBA_BIT,
      ffi::GLX_RED_SIZE,             8,
      ffi::GLX_GREEN_SIZE,           8,
      ffi::GLX_BLUE_SIZE,            8,
      ffi::GLX_ALPHA_SIZE,           8,

      ffi::GLX_DOUBLEBUFFER,         1,

      ffi::GLX_DEPTH_SIZE,           0,
      ffi::GLX_STENCIL_SIZE,         0,

      0
   ];

   let mut num_config: c_int = unsafe {
      mem::uninitialized()
   };

   let result = unsafe {
      ffi::glXChooseFBConfig(
         display.ptr,
         screen_id.screen,
         config_attribs.as_ptr() as *const _,
         &mut num_config
      )
   };

   if result.is_null() {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "Choosing GLX config failed".to_string()
      ));
   }

   if num_config == 0 {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "Failed to find suitable GLXFBConfig".to_string()
      ));
   }

   let chosen = unsafe { *result };

   x11::xfree(result as *mut c_void);

   Ok(Config {
      ptr: chosen
   })
}

pub fn get_visual(display: &Display, config: &Config) -> Result<XVisualInfo, RuntimeError> {
   let info_ptr = unsafe {
      ffi::glXGetVisualFromFBConfig(display.ptr, config.ptr)
   };

   if info_ptr.is_null() {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "Failed to get visual from FBConfig".to_string()
      ));
   }

   let info = unsafe {
      ptr::read(info_ptr as *const _)
   };

   x11::xfree(info_ptr as *mut c_void);

   Ok(info)
}

pub fn create_new_context(display: &Display, config: &Config) -> Result<Context, RuntimeError> {
   let context = unsafe {
      ffi::glXCreateNewContext(
         display.ptr,
         config.ptr,
         ffi::GLX_RGBA_TYPE,
         ptr::null_mut(),
         1
      )
   };
   if context.is_null() {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "glXCreateNewContext failed".to_string()
      ));
   }

   Ok(Context {
      ptr: context
   })
}

pub fn create_rendering_area(
   display: &Display,
   config: &Config,
   native_window: GLXNativeWindowType
) -> Result<ffi::GLXWindow, RuntimeError> {
   let win = unsafe {
      ffi::glXCreateWindow(
         display.ptr,
         config.ptr,
         native_window,
         ptr::null()
      )
   };

   if win == 0 {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "glXCreateWindow failed".to_string()
      ));
   }

   Ok(win)
}

pub fn make_current(
   display: &Display,
   rendering_area: ffi::GLXWindow,
   context: &Context,
) -> VoidResult {

   let made_current = unsafe {
      ffi::glXMakeContextCurrent(
         display.ptr,
         rendering_area,
         rendering_area,
         context.ptr
      )
   };

   if made_current == 0 {
      return Err(RuntimeError::new(
         ErrorKind::GLX,
         "glXMakeContextCurrent failed".to_string()
      ));
   }

   Ok(())
}

pub fn swap_buffers(
   display: &Display,
   rendering_area: ffi::GLXWindow,
) -> VoidResult {

   unsafe {
      ffi::glXSwapBuffers(display.ptr, rendering_area)
   };

   Ok(())
}

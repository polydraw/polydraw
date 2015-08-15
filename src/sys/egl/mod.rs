#![cfg(target_os = "linux")]

pub mod ffi;

use std::mem;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::x11::ffi::Display as X11Display;
use super::xcb::ffi::xcb_window_t;

use super::x11;
use super::utils::fn_ptr::{FnPtrLoader, FnPtr};

pub type EGLNativeDisplayType = *mut X11Display;
pub type EGLNativeWindowType = xcb_window_t;

pub struct Loader;

impl Loader {
   pub fn new() -> Self {
      Loader
   }
}

impl FnPtrLoader for Loader {
   fn get_proc_addr(&self, name: &str) -> FnPtr {
      let cname = CString::new(name).unwrap().as_ptr();

      let addr = unsafe {
         ffi::eglGetProcAddress(cname)
      };
      println!("{}: {:?}", name, addr);

      addr
   }
}

pub type NativeDisplay = x11::Display;

pub enum API {
    OpenGLES,
    OpenVG,
    OpenGL,
}

impl Into<ffi::EGLenum> for API {
   #[inline]
   fn into(self) -> ffi::EGLenum {
      match self {
         API::OpenGLES => ffi::EGL_OPENGL_ES_API,
         API::OpenVG => ffi::EGL_OPENVG_API,
         API::OpenGL => ffi::EGL_OPENGL_API,
      }
   }
}

pub struct Display {
   pub ptr: ffi::EGLDisplay
}

pub struct Config {
   pub ptr: ffi::EGLConfig
}

pub struct Context {
   pub ptr: ffi::EGLContext
}

pub struct Surface {
   pub ptr: ffi::EGLSurface
}

pub struct Version {
   pub major: ffi::EGLint,
   pub minor: ffi::EGLint,
}

pub fn bind_api(api: API) -> bool {
   unsafe {
      ffi::eglBindAPI(api.into()) != 0
   }
}

pub fn get_display(display: &NativeDisplay) -> Display {
   Display {
      ptr: unsafe {
         ffi::eglGetDisplay(display.ptr)
      }
   }
}

pub fn initialize(display: &Display) -> Result<Version, RuntimeError> {
   let mut major: ffi::EGLint = unsafe {
      mem::uninitialized()
   };
   let mut minor: ffi::EGLint = unsafe {
      mem::uninitialized()
   };

   let result = unsafe {
      ffi::eglInitialize(display.ptr, &mut major, &mut minor)
   };

   match result {
      ffi::EGL_FALSE => {
         return Err(RuntimeError::new(
            ErrorKind::EGL,
            "eglInitialize failed".to_string()
         ));
      },
      ffi::EGL_BAD_DISPLAY => {
         return Err(RuntimeError::new(
            ErrorKind::EGL,
            "Not an EGL display connection".to_string()
         ));
      },
      ffi::EGL_NOT_INITIALIZED => {
         return Err(RuntimeError::new(
            ErrorKind::EGL,
            "Display cannot be initialized".to_string()
         ));
      },
      ffi::EGL_TRUE => {
         return Ok(Version {
            major: major,
            minor: minor
         });
      },
      _ => {
         return Err(RuntimeError::new(
            ErrorKind::EGL,
            "Unknown eglInitialize error".to_string()
         ));
      }
   }
}

pub fn choose_config(display: &Display) -> Result<Config, RuntimeError> {
   let config_attribs = [
      ffi::EGL_COLOR_BUFFER_TYPE,    ffi::EGL_RGB_BUFFER,
      ffi::EGL_BUFFER_SIZE,          32,
      ffi::EGL_RED_SIZE,             8,
      ffi::EGL_GREEN_SIZE,           8,
      ffi::EGL_BLUE_SIZE,            8,
      ffi::EGL_ALPHA_SIZE,           8,

      ffi::EGL_DEPTH_SIZE,           24,
      ffi::EGL_STENCIL_SIZE,         8,

      ffi::EGL_SAMPLE_BUFFERS,       0,
      ffi::EGL_SAMPLES,              0,

      ffi::EGL_SURFACE_TYPE,         ffi::EGL_WINDOW_BIT,
      ffi::EGL_RENDERABLE_TYPE,      ffi::EGL_OPENGL_BIT,

      ffi::EGL_NONE
   ];

   let mut num_config: ffi::EGLint = unsafe {
      mem::uninitialized()
   };

   let mut configs: [ffi::EGLConfig; 64] = unsafe {
      mem::uninitialized()
   };

   let result = unsafe {
      ffi::eglChooseConfig(
         display.ptr,
         config_attribs.as_ptr() as *const _,
         configs.as_mut_ptr() as *mut *mut _,
         64,
         &mut num_config
      )
   };

   if result != ffi::EGL_TRUE {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "Choosing config failed".to_string()
      ));
   }

   if num_config == 0 {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "Failed to find suitable EGLConfig".to_string()
      ));
   }

   Ok(Config {
      ptr: configs[0]
   })
}

pub fn create_context(display: &Display, config: &Config) -> Result<Context, RuntimeError> {
   let context_attribs = [ffi::EGL_NONE];

   let context = unsafe {
      ffi::eglCreateContext(
         display.ptr,
         config.ptr as *mut _,
         ffi::EGL_NO_CONTEXT as *mut _,
         context_attribs.as_ptr() as *const _,
      )
   };
   if context.is_null() {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglCreateContext failed".to_string()
      ));
   }

   Ok(Context {
      ptr: context
   })
}

pub fn create_window_surface(
   display: &Display,
   config: &Config,
   window: &EGLNativeWindowType
) -> Result<Surface, RuntimeError> {

   let surface_attribs = [
      ffi::EGL_RENDER_BUFFER, ffi::EGL_BACK_BUFFER,
      ffi::EGL_NONE
   ];

   let surface = unsafe {
      ffi::eglCreateWindowSurface(
         display.ptr,
         config.ptr as *mut _,
         *window,
         surface_attribs.as_ptr() as *const _,
      )
   };
   if surface.is_null() {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglCreateWindowSurface failed".to_string()
      ));
   }

   Ok(Surface {
      ptr: surface
   })
}

pub fn make_current(
   display: &Display,
   draw: &Surface,
   read: &Surface,
   context: &Context,
) -> Result<(), RuntimeError> {

   let made_current = unsafe {
      ffi::eglMakeCurrent(
         display.ptr,
         draw.ptr,
         read.ptr,
         context.ptr
      )
   };

   if made_current != ffi::EGL_TRUE {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglMakeCurrent failed".to_string()
      ));
   }

   Ok(())
}

pub fn query_context(
   display: &Display,
   context: &Context,
) -> Result<(), RuntimeError> {

   let mut render_buffer: ffi::EGLint = unsafe { mem::uninitialized() };

   let result = unsafe {
      ffi::eglQueryContext(
         display.ptr,
         context.ptr,
         ffi::EGL_RENDER_BUFFER as i32,
         &mut render_buffer
      )
   };

   if result != ffi::EGL_TRUE {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglQueyContext (EGL_RENDER_BUFFER) failed".to_string()
      ));
   }

   if render_buffer == ffi::EGL_SINGLE_BUFFER as i32 {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "EGL surface is single buffered".to_string()
      ));
   }

   Ok(())
}

pub fn swap_buffers(
   display: &Display,
   surface: &Surface
) -> Result<(), RuntimeError> {

   let result = unsafe {
      ffi::eglSwapBuffers(display.ptr, surface.ptr)
   };

   if result != ffi::EGL_TRUE {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglSwapBuffers failed".to_string()
      ));
   }

   Ok(())
}

pub fn swap_interval(
   display: &Display,
   interval: ffi::c_int
) -> Result<(), RuntimeError> {

   let result = unsafe {
      ffi::eglSwapInterval(display.ptr, interval)
   };

   if result != ffi::EGL_TRUE {
      return Err(RuntimeError::new(
         ErrorKind::EGL,
         "eglSwapInterval failed".to_string()
      ));
   }

   Ok(())
}

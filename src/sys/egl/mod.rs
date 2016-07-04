#![cfg(target_os = "linux")]

pub mod ffi;

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::iter::Iterator;

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

#[inline]
fn egl_error<T>(message: &str) -> Result<T, RuntimeError> {
   Err(
      RuntimeError::new(
         ErrorKind::EGL,
         message.to_string()
      )
   )
}

#[inline]
fn egl_result<T>(value: T) -> Result<T, RuntimeError> {
   let result = unsafe {
      ffi::eglGetError()
   };

   match result {
      ffi::EGL_SUCCESS => Ok(value),
      ffi::EGL_NOT_INITIALIZED => egl_error(
         "EGL is not initialized"
      ),
      ffi::EGL_BAD_ACCESS => egl_error(
         "EGL cannot access a requested resource"
      ),
      ffi::EGL_BAD_ALLOC => egl_error(
         "EGL failed to allocate resources for the requested operation"
      ),
      ffi::EGL_BAD_ATTRIBUTE => egl_error(
         "Unrecognized attribute or attribute value"
      ),
      ffi::EGL_BAD_CONTEXT => egl_error(
         "An EGLContext argument does not name a valid EGL rendering context"
      ),
      ffi::EGL_BAD_CONFIG => egl_error(
         "An EGLConfig argument does not name a valid EGL frame buffer configuration"
      ),
      ffi::EGL_BAD_CURRENT_SURFACE => egl_error(
         "The current surface is a window, pixel buffer or pixmap that is no longer valid"
      ),
      ffi::EGL_BAD_DISPLAY => egl_error(
         "An EGLDisplay argument does not name a valid EGL display connection"
      ),
      ffi::EGL_BAD_SURFACE => egl_error(
         "An EGLSurface argument does not name a valid surface configured for GL rendering"
      ),
      ffi::EGL_BAD_MATCH => egl_error(
         "Arguments are inconsistent"
      ),
      ffi::EGL_BAD_PARAMETER => egl_error(
         "One or more argument values are invalid"
      ),
      ffi::EGL_BAD_NATIVE_PIXMAP => egl_error(
         "A NativePixmapType argument does not refer to a valid native pixmap"
      ),
      ffi::EGL_BAD_NATIVE_WINDOW => egl_error(
         "A NativeWindowType argument does not refer to a valid native window"
      ),
      ffi::EGL_CONTEXT_LOST => egl_error(
         "A power management event has occurred - the application must reinitialise OpenGL ES"
      ),
      _ => egl_error("Unknown EGL error")
   }
}

pub fn bind_api(api: API) -> Result<(), RuntimeError> {
   let result = unsafe {
      ffi::eglBindAPI(api.into())
   };

   if result == 0 {
      return egl_error("eglBindAPI failed");
   }

   egl_result(())
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
      ffi::EGL_FALSE => egl_error("eglInitialize failed"),
      _ => egl_result(Version {major: major, minor: minor}),
   }
}

pub fn configs(display: &Display) -> Result<Vec<Config>, RuntimeError> {
   let mut num_config = unsafe { mem::uninitialized() };

   let result = unsafe {
      ffi::eglGetConfigs(
         display.ptr,
         ptr::null_mut(),
         0,
         &mut num_config,
      )
   };

   if result == 0 {
      return egl_error("Getting configs count eglGetConfigs failed");
   }

   let mut config_ptrs = Vec::with_capacity(num_config as usize);

   let result = unsafe {
      ffi::eglGetConfigs(
         display.ptr,
         config_ptrs.as_mut_ptr(),
         config_ptrs.capacity() as ffi::EGLint,
         &mut num_config
      )
   };

   unsafe {
      config_ptrs.set_len(num_config as usize)
   };

   if result == 0 {
      return egl_error("eglGetConfigs failed");
   }

   let configs = config_ptrs.iter().map(|&ptr| Config {ptr: ptr}).collect();

   for config in &configs {
      print_config(display, config);
   }

   Ok(configs)
}

pub fn choose_config(display: &Display) -> Result<Config, RuntimeError> {
   let config_attribs = [
      ffi::EGL_COLOR_BUFFER_TYPE,    ffi::EGL_RGB_BUFFER,
      ffi::EGL_BUFFER_SIZE,          32,
      ffi::EGL_RED_SIZE,             8,
      ffi::EGL_GREEN_SIZE,           8,
      ffi::EGL_BLUE_SIZE,            8,
      ffi::EGL_ALPHA_SIZE,           8,

      ffi::EGL_DEPTH_SIZE,           0,
      ffi::EGL_STENCIL_SIZE,         0,

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
      return egl_error("Choosing EGL config failed");
   }

   if num_config == 0 {
      return egl_error("Failed to find suitable EGLConfig");
   }

   egl_result(Config {
      ptr: configs[0]
   })
}

pub fn config_attrib(
   display: &Display,
   config: &Config,
   attribute: ffi::EGLint
) -> Result<ffi::EGLint, RuntimeError> {

   let mut value: ffi::EGLint = unsafe { mem::uninitialized() };

   let result = unsafe {
      ffi::eglGetConfigAttrib(display.ptr, config.ptr, attribute, &mut value)
   };

   if result != ffi::EGL_TRUE {
      return egl_error("eglGetConfigAttrib failed");
   }

   egl_result(value)
}

pub fn print_config(display: &Display, config: &Config) {
   println!("-------------------------");

   pattr(display, config, "EGL_CONFIG_ID", ffi::EGL_CONFIG_ID);
   pattr(display, config, "EGL_COLOR_BUFFER_TYPE", ffi::EGL_COLOR_BUFFER_TYPE);
   pattr(display, config, "EGL_RENDERABLE_TYPE", ffi::EGL_RENDERABLE_TYPE);
   pattr(display, config, "EGL_SURFACE_TYPE", ffi::EGL_SURFACE_TYPE);
   pattr(display, config, "EGL_TRANSPARENT_TYPE", ffi::EGL_TRANSPARENT_TYPE);
   pattr(display, config, "EGL_NATIVE_VISUAL_TYPE", ffi::EGL_NATIVE_VISUAL_TYPE);
   pattr(display, config, "EGL_NATIVE_VISUAL_ID", ffi::EGL_NATIVE_VISUAL_ID);
   pattr(display, config, "EGL_BUFFER_SIZE", ffi::EGL_BUFFER_SIZE);
   pattr(display, config, "EGL_LUMINANCE_SIZE", ffi::EGL_LUMINANCE_SIZE);
   pattr(display, config, "EGL_DEPTH_SIZE", ffi::EGL_DEPTH_SIZE);
   pattr(display, config, "EGL_STENCIL_SIZE", ffi::EGL_STENCIL_SIZE);
   pattr(display, config, "EGL_RED_SIZE", ffi::EGL_RED_SIZE);
   pattr(display, config, "EGL_GREEN_SIZE", ffi::EGL_GREEN_SIZE);
   pattr(display, config, "EGL_BLUE_SIZE", ffi::EGL_BLUE_SIZE);
   pattr(display, config, "EGL_ALPHA_SIZE", ffi::EGL_ALPHA_SIZE);
   pattr(display, config, "EGL_ALPHA_MASK_SIZE", ffi::EGL_ALPHA_MASK_SIZE);
   pattr(display, config, "EGL_BIND_TO_TEXTURE_RGB", ffi::EGL_BIND_TO_TEXTURE_RGB);
   pattr(display, config, "EGL_BIND_TO_TEXTURE_RGBA", ffi::EGL_BIND_TO_TEXTURE_RGBA);
   pattr(display, config, "EGL_CONFIG_CAVEAT", ffi::EGL_CONFIG_CAVEAT);
   pattr(display, config, "EGL_CONFORMANT", ffi::EGL_CONFORMANT);
   pattr(display, config, "EGL_LEVEL", ffi::EGL_LEVEL);
   pattr(display, config, "EGL_MAX_PBUFFER_WIDTH", ffi::EGL_MAX_PBUFFER_WIDTH);
   pattr(display, config, "EGL_MAX_PBUFFER_HEIGHT", ffi::EGL_MAX_PBUFFER_HEIGHT);
   pattr(display, config, "EGL_MAX_PBUFFER_PIXELS", ffi::EGL_MAX_PBUFFER_PIXELS);
   pattr(display, config, "EGL_MAX_SWAP_INTERVAL", ffi::EGL_MAX_SWAP_INTERVAL);
   pattr(display, config, "EGL_MIN_SWAP_INTERVAL", ffi::EGL_MIN_SWAP_INTERVAL);
   pattr(display, config, "EGL_NATIVE_RENDERABLE", ffi::EGL_NATIVE_RENDERABLE);
   pattr(display, config, "EGL_SAMPLE_BUFFERS", ffi::EGL_SAMPLE_BUFFERS);
   pattr(display, config, "EGL_SAMPLES", ffi::EGL_SAMPLES);
   pattr(display, config, "EGL_TRANSPARENT_RED_VALUE", ffi::EGL_TRANSPARENT_RED_VALUE);
   pattr(display, config, "EGL_TRANSPARENT_GREEN_VALUE", ffi::EGL_TRANSPARENT_GREEN_VALUE);
   pattr(display, config, "EGL_TRANSPARENT_BLUE_VALUE", ffi::EGL_TRANSPARENT_BLUE_VALUE);

   println!("");
}

pub fn pattr(display: &Display, config: &Config, name: &str, attribute: ffi::EGLenum) {
   println!(
      "{}: {}", name, config_attrib(display, config, attribute as ffi::EGLint).unwrap()
   );
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
      return egl_error("eglCreateContext failed");
   }

   egl_result(Context {
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
      return egl_error("eglCreateWindowSurface failed");
   }

   egl_result(Surface {
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

   match made_current {
      ffi::EGL_FALSE => egl_error("eglMakeCurrent failed"),
      _ => egl_result(())
   }
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
      return egl_error("eglQueyContext (EGL_RENDER_BUFFER) failed");
   }

   if render_buffer == ffi::EGL_SINGLE_BUFFER as i32 {
      return egl_error("EGL surface is single buffered");
   }

   egl_result(())
}

pub fn swap_buffers(
   display: &Display,
   surface: &Surface
) -> Result<(), RuntimeError> {

   let result = unsafe {
      ffi::eglSwapBuffers(display.ptr, surface.ptr)
   };

   match result {
      ffi::EGL_FALSE => egl_error("eglSwapBuffers failed"),
      _ => egl_result(())
   }
}

pub fn swap_interval(
   display: &Display,
   interval: ffi::c_int
) -> Result<(), RuntimeError> {

   let result = unsafe {
      ffi::eglSwapInterval(display.ptr, interval)
   };

   match result {
      ffi::EGL_FALSE => egl_error("eglSwapInterval failed"),
      _ => egl_result(())
   }
}

impl Display {
   pub fn from_native(native_display: &NativeDisplay) -> Result<Self, RuntimeError> {
      let ptr = unsafe {
         ffi::eglGetDisplay(native_display.ptr)
      };

      if ptr.is_null() {
         return egl_error("eglGetDisplay failed");
      }

      egl_result(Display {
         ptr: ptr,
      })
   }
}

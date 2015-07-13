use ::os::x11::ffi::Display as X11Display;
use ::os::xcb::ffi::xcb_window_t;

pub type EGLNativeDisplayType = *mut X11Display;
pub type EGLNativeWindowType = xcb_window_t;

pub mod ffi {
   #![allow(non_camel_case_types)]

   use super::{EGLNativeDisplayType, EGLNativeWindowType};

   use libc::{
      c_char, c_int, c_uint, c_void
   };

   pub type khronos_int32_t = c_int;

   pub type EGLenum = c_uint;
   pub type EGLint = khronos_int32_t;
   pub type EGLBoolean = c_uint;
   pub type EGLDisplay = *mut c_void;
   pub type EGLConfig = *mut c_void;
   pub type EGLContext = *mut c_void;
   pub type EGLSurface = *mut c_void;

   pub const EGL_FALSE:                        EGLenum = 0;
   pub const EGL_TRUE:                         EGLenum = 1;

   pub const EGL_DEFAULT_DISPLAY:              EGLNativeDisplayType = 0 as EGLNativeDisplayType;
   pub const EGL_NO_CONTEXT:                   EGLContext = 0 as EGLContext;
   pub const EGL_NO_DISPLAY:                   EGLDisplay = 0 as EGLDisplay;
   pub const EGL_NO_SURFACE:                   EGLSurface = 0 as EGLSurface;

   pub const EGL_DONT_CARE:                    EGLint = -1;

   pub const EGL_SUCCESS:                      EGLenum = 0x3000;
   pub const EGL_NOT_INITIALIZED:              EGLenum = 0x3001;
   pub const EGL_BAD_ACCESS:                   EGLenum = 0x3002;
   pub const EGL_BAD_ALLOC:                    EGLenum = 0x3003;
   pub const EGL_BAD_ATTRIBUTE:                EGLenum = 0x3004;
   pub const EGL_BAD_CONFIG:                   EGLenum = 0x3005;
   pub const EGL_BAD_CONTEXT:                  EGLenum = 0x3006;
   pub const EGL_BAD_CURRENT_SURFACE:          EGLenum = 0x3007;
   pub const EGL_BAD_DISPLAY:                  EGLenum = 0x3008;
   pub const EGL_BAD_MATCH:                    EGLenum = 0x3009;
   pub const EGL_BAD_NATIVE_PIXMAP:            EGLenum = 0x300A;
   pub const EGL_BAD_NATIVE_WINDOW:            EGLenum = 0x300B;
   pub const EGL_BAD_PARAMETER:                EGLenum = 0x300C;
   pub const EGL_BAD_SURFACE:                  EGLenum = 0x300D;
   pub const EGL_CONTEXT_LOST:                 EGLenum = 0x300E;

   pub const EGL_BUFFER_SIZE:                  EGLenum = 0x3020;
   pub const EGL_ALPHA_SIZE:                   EGLenum = 0x3021;
   pub const EGL_BLUE_SIZE:                    EGLenum = 0x3022;
   pub const EGL_GREEN_SIZE:                   EGLenum = 0x3023;
   pub const EGL_RED_SIZE:                     EGLenum = 0x3024;
   pub const EGL_DEPTH_SIZE:                   EGLenum = 0x3025;
   pub const EGL_STENCIL_SIZE:                 EGLenum = 0x3026;
   pub const EGL_CONFIG_CAVEAT:                EGLenum = 0x3027;
   pub const EGL_CONFIG_ID:                    EGLenum = 0x3028;
   pub const EGL_LEVEL:                        EGLenum = 0x3029;
   pub const EGL_MAX_PBUFFER_HEIGHT:           EGLenum = 0x302A;
   pub const EGL_MAX_PBUFFER_PIXELS:           EGLenum = 0x302B;
   pub const EGL_MAX_PBUFFER_WIDTH:            EGLenum = 0x302C;
   pub const EGL_NATIVE_RENDERABLE:            EGLenum = 0x302D;
   pub const EGL_NATIVE_VISUAL_ID:             EGLenum = 0x302E;
   pub const EGL_NATIVE_VISUAL_TYPE:           EGLenum = 0x302F;
   pub const EGL_SAMPLES:                      EGLenum = 0x3031;
   pub const EGL_SAMPLE_BUFFERS:               EGLenum = 0x3032;
   pub const EGL_SURFACE_TYPE:                 EGLenum = 0x3033;
   pub const EGL_TRANSPARENT_TYPE:             EGLenum = 0x3034;
   pub const EGL_TRANSPARENT_BLUE_VALUE:       EGLenum = 0x3035;
   pub const EGL_TRANSPARENT_GREEN_VALUE:      EGLenum = 0x3036;
   pub const EGL_TRANSPARENT_RED_VALUE:        EGLenum = 0x3037;
   pub const EGL_NONE:                         EGLenum = 0x3038;
   pub const EGL_BIND_TO_TEXTURE_RGB:          EGLenum = 0x3039;
   pub const EGL_BIND_TO_TEXTURE_RGBA:         EGLenum = 0x303A;
   pub const EGL_MIN_SWAP_INTERVAL:            EGLenum = 0x303B;
   pub const EGL_MAX_SWAP_INTERVAL:            EGLenum = 0x303C;
   pub const EGL_LUMINANCE_SIZE:               EGLenum = 0x303D;
   pub const EGL_ALPHA_MASK_SIZE:              EGLenum = 0x303E;
   pub const EGL_COLOR_BUFFER_TYPE:            EGLenum = 0x303F;
   pub const EGL_RENDERABLE_TYPE:              EGLenum = 0x3040;
   pub const EGL_MATCH_NATIVE_PIXMAP:          EGLenum = 0x3041;
   pub const EGL_CONFORMANT:                   EGLenum = 0x3042;

   pub const EGL_SLOW_CONFIG:                  EGLenum = 0x3050;
   pub const EGL_NON_CONFORMANT_CONFIG:        EGLenum = 0x3051;
   pub const EGL_TRANSPARENT_RGB:              EGLenum = 0x3052;
   pub const EGL_RGB_BUFFER:                   EGLenum = 0x308E;
   pub const EGL_LUMINANCE_BUFFER:             EGLenum = 0x308F;

   pub const EGL_HEIGHT:                       EGLenum = 0x3056;
   pub const EGL_WIDTH:                        EGLenum = 0x3057;
   pub const EGL_LARGEST_PBUFFER:              EGLenum = 0x3058;
   pub const EGL_TEXTURE_FORMAT:               EGLenum = 0x3080;
   pub const EGL_TEXTURE_TARGET:               EGLenum = 0x3081;
   pub const EGL_MIPMAP_TEXTURE:               EGLenum = 0x3082;
   pub const EGL_MIPMAP_LEVEL:                 EGLenum = 0x3083;
   pub const EGL_RENDER_BUFFER:                EGLenum = 0x3086;
   pub const EGL_VG_COLORSPACE:                EGLenum = 0x3087;
   pub const EGL_VG_ALPHA_FORMAT:              EGLenum = 0x3088;
   pub const EGL_HORIZONTAL_RESOLUTION:        EGLenum = 0x3090;
   pub const EGL_VERTICAL_RESOLUTION:          EGLenum = 0x3091;
   pub const EGL_PIXEL_ASPECT_RATIO:           EGLenum = 0x3092;
   pub const EGL_SWAP_BEHAVIOR:                EGLenum = 0x3093;
   pub const EGL_MULTISAMPLE_RESOLVE:          EGLenum = 0x3099;

   pub const EGL_BACK_BUFFER:                  EGLenum = 0x3084;
   pub const EGL_SINGLE_BUFFER:                EGLenum = 0x3085;

   pub const EGL_OPENGL_ES_API:                EGLenum = 0x30A0;
   pub const EGL_OPENVG_API:                   EGLenum = 0x30A1;
   pub const EGL_OPENGL_API:                   EGLenum = 0x30A2;

   pub const EGL_PBUFFER_BIT:                  EGLenum = 0x0001;
   pub const EGL_PIXMAP_BIT:                   EGLenum = 0x0002;
   pub const EGL_WINDOW_BIT:                   EGLenum = 0x0004;
   pub const EGL_VG_COLORSPACE_LINEAR_BIT:     EGLenum = 0x0020;
   pub const EGL_VG_ALPHA_FORMAT_PRE_BIT:      EGLenum = 0x0040;
   pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT:  EGLenum = 0x0200;
   pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT:  EGLenum = 0x0400;

   pub const EGL_OPENGL_ES_BIT:                EGLenum = 0x0001;
   pub const EGL_OPENVG_BIT:                   EGLenum = 0x0002;
   pub const EGL_OPENGL_ES2_BIT:               EGLenum = 0x0004;
   pub const EGL_OPENGL_BIT:                   EGLenum = 0x0008;

   pub type __eglMustCastToProperFunctionPointerType =
   //   Option<extern "C" fn() -> ()>;
      *const c_void;

   #[link(name="EGL")]
   extern "C" {
      pub fn eglBindAPI(
         api: EGLenum
      ) -> EGLBoolean;

      pub fn eglGetDisplay(
         display_id: EGLNativeDisplayType
      ) -> EGLDisplay;

      pub fn eglInitialize(
         dpy: EGLDisplay,
         major: *mut EGLint,
         minor: *mut EGLint
      ) -> EGLBoolean;

      pub fn eglChooseConfig(
         dpy: EGLDisplay,
         attrib_list: *const EGLint,
         configs: *mut EGLConfig,
         config_size: EGLint,
         num_config: *mut EGLint
      ) -> EGLBoolean;

      pub fn eglGetProcAddress(
         procname: *const c_char
      ) -> __eglMustCastToProperFunctionPointerType;

      pub fn eglCreateContext(
         dpy: EGLDisplay,
         config: EGLConfig,
         share_context: EGLContext,
         attrib_list: *const EGLint
      ) -> EGLContext;

      pub fn eglCreateWindowSurface(
         dpy: EGLDisplay,
         config: EGLConfig,
         win: EGLNativeWindowType,
         attrib_list: *const EGLint
      ) -> EGLSurface;

      pub fn eglMakeCurrent(
         dpy: EGLDisplay,
         draw: EGLSurface,
         read: EGLSurface,
         ctx: EGLContext
      ) -> EGLBoolean;

      pub fn eglQueryContext(
         dpy: EGLDisplay,
         ctx: EGLContext,
         attribute: EGLint,
         value: *mut EGLint
      ) -> EGLBoolean;

      pub fn eglSwapBuffers(
         dpy: EGLDisplay,
         surface: EGLSurface
      ) -> EGLBoolean;
   }
}

use std::mem;

use ::error::{RuntimeError, ErrorKind};
use ::os::x11;

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

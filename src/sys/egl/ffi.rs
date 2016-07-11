#![allow(non_camel_case_types)]

use super::{EGLNativeDisplayType, EGLNativeWindowType};

pub use libc::{
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

pub const EGL_SUCCESS:                      EGLint = 0x3000;
pub const EGL_NOT_INITIALIZED:              EGLint = 0x3001;
pub const EGL_BAD_ACCESS:                   EGLint = 0x3002;
pub const EGL_BAD_ALLOC:                    EGLint = 0x3003;
pub const EGL_BAD_ATTRIBUTE:                EGLint = 0x3004;
pub const EGL_BAD_CONFIG:                   EGLint = 0x3005;
pub const EGL_BAD_CONTEXT:                  EGLint = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE:          EGLint = 0x3007;
pub const EGL_BAD_DISPLAY:                  EGLint = 0x3008;
pub const EGL_BAD_MATCH:                    EGLint = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP:            EGLint = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW:            EGLint = 0x300B;
pub const EGL_BAD_PARAMETER:                EGLint = 0x300C;
pub const EGL_BAD_SURFACE:                  EGLint = 0x300D;
pub const EGL_CONTEXT_LOST:                 EGLint = 0x300E;

pub const EGL_BUFFER_SIZE:                  EGLint = 0x3020;
pub const EGL_ALPHA_SIZE:                   EGLint = 0x3021;
pub const EGL_BLUE_SIZE:                    EGLint = 0x3022;
pub const EGL_GREEN_SIZE:                   EGLint = 0x3023;
pub const EGL_RED_SIZE:                     EGLint = 0x3024;
pub const EGL_DEPTH_SIZE:                   EGLint = 0x3025;
pub const EGL_STENCIL_SIZE:                 EGLint = 0x3026;
pub const EGL_CONFIG_CAVEAT:                EGLint = 0x3027;
pub const EGL_CONFIG_ID:                    EGLint = 0x3028;
pub const EGL_LEVEL:                        EGLint = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT:           EGLint = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS:           EGLint = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH:            EGLint = 0x302C;
pub const EGL_NATIVE_RENDERABLE:            EGLint = 0x302D;
pub const EGL_NATIVE_VISUAL_ID:             EGLint = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE:           EGLint = 0x302F;
pub const EGL_SAMPLES:                      EGLint = 0x3031;
pub const EGL_SAMPLE_BUFFERS:               EGLint = 0x3032;
pub const EGL_SURFACE_TYPE:                 EGLint = 0x3033;
pub const EGL_TRANSPARENT_TYPE:             EGLint = 0x3034;
pub const EGL_TRANSPARENT_BLUE_VALUE:       EGLint = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE:      EGLint = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE:        EGLint = 0x3037;
pub const EGL_NONE:                         EGLint = 0x3038;
pub const EGL_BIND_TO_TEXTURE_RGB:          EGLint = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA:         EGLint = 0x303A;
pub const EGL_MIN_SWAP_INTERVAL:            EGLint = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL:            EGLint = 0x303C;
pub const EGL_LUMINANCE_SIZE:               EGLint = 0x303D;
pub const EGL_ALPHA_MASK_SIZE:              EGLint = 0x303E;
pub const EGL_COLOR_BUFFER_TYPE:            EGLint = 0x303F;
pub const EGL_RENDERABLE_TYPE:              EGLint = 0x3040;
pub const EGL_MATCH_NATIVE_PIXMAP:          EGLint = 0x3041;
pub const EGL_CONFORMANT:                   EGLint = 0x3042;

pub const EGL_SLOW_CONFIG:                  EGLint = 0x3050;
pub const EGL_NON_CONFORMANT_CONFIG:        EGLint = 0x3051;
pub const EGL_TRANSPARENT_RGB:              EGLint = 0x3052;
pub const EGL_RGB_BUFFER:                   EGLint = 0x308E;
pub const EGL_LUMINANCE_BUFFER:             EGLint = 0x308F;

pub const EGL_HEIGHT:                       EGLenum = 0x3056;
pub const EGL_WIDTH:                        EGLenum = 0x3057;
pub const EGL_LARGEST_PBUFFER:              EGLenum = 0x3058;
pub const EGL_TEXTURE_FORMAT:               EGLenum = 0x3080;
pub const EGL_TEXTURE_TARGET:               EGLenum = 0x3081;
pub const EGL_MIPMAP_TEXTURE:               EGLenum = 0x3082;
pub const EGL_MIPMAP_LEVEL:                 EGLenum = 0x3083;
pub const EGL_BACK_BUFFER:                  EGLenum = 0x3084;
pub const EGL_SINGLE_BUFFER:                EGLenum = 0x3085;
pub const EGL_RENDER_BUFFER:                EGLenum = 0x3086;
pub const EGL_VG_COLORSPACE:                EGLenum = 0x3087;
pub const EGL_VG_ALPHA_FORMAT:              EGLenum = 0x3088;
pub const EGL_HORIZONTAL_RESOLUTION:        EGLenum = 0x3090;
pub const EGL_VERTICAL_RESOLUTION:          EGLenum = 0x3091;
pub const EGL_PIXEL_ASPECT_RATIO:           EGLenum = 0x3092;
pub const EGL_SWAP_BEHAVIOR:                EGLenum = 0x3093;
pub const EGL_CONTEXT_CLIENT_VERSION:       EGLenum = 0x3098;
pub const EGL_MULTISAMPLE_RESOLVE:          EGLenum = 0x3099;

pub const EGL_OPENGL_ES_API:                EGLenum = 0x30A0;
pub const EGL_OPENVG_API:                   EGLenum = 0x30A1;
pub const EGL_OPENGL_API:                   EGLenum = 0x30A2;

pub const EGL_PBUFFER_BIT:                  EGLint = 0x0001;
pub const EGL_PIXMAP_BIT:                   EGLint = 0x0002;
pub const EGL_WINDOW_BIT:                   EGLint = 0x0004;
pub const EGL_VG_COLORSPACE_LINEAR_BIT:     EGLint = 0x0020;
pub const EGL_VG_ALPHA_FORMAT_PRE_BIT:      EGLint = 0x0040;
pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT:  EGLint = 0x0200;
pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT:  EGLint = 0x0400;

pub const EGL_OPENGL_ES_BIT:                EGLint = 0x0001;
pub const EGL_OPENVG_BIT:                   EGLint = 0x0002;
pub const EGL_OPENGL_ES2_BIT:               EGLint = 0x0004;
pub const EGL_OPENGL_BIT:                   EGLint = 0x0008;

pub type __eglMustCastToProperFunctionPointerType =
//   Option<extern "C" fn() -> ()>;
   *const c_void;

#[link(name="EGL")]
extern "C" {
   pub fn eglGetError() -> EGLint;

   pub fn eglBindAPI(
      api: EGLenum
   ) -> EGLBoolean;

   pub fn eglGetDisplay(
      display_id: EGLNativeDisplayType
   ) -> EGLDisplay;

   pub fn eglInitialize(
      display: EGLDisplay,
      major: *mut EGLint,
      minor: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglGetConfigs(
      display: EGLDisplay ,
      configs: *mut EGLConfig,
      config_size: EGLint,
      num_config: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglChooseConfig(
      display: EGLDisplay,
      attrib_list: *const EGLint,
      configs: *mut EGLConfig,
      config_size: EGLint,
      num_config: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglGetConfigAttrib(
      display: EGLDisplay,
      config: EGLConfig,
      attribute: EGLint,
      value: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglGetProcAddress(
      procname: *const c_char
   ) -> __eglMustCastToProperFunctionPointerType;

   pub fn eglCreateContext(
      display: EGLDisplay,
      config: EGLConfig,
      share_context: EGLContext,
      attrib_list: *const EGLint
   ) -> EGLContext;

   pub fn eglCreateWindowSurface(
      display: EGLDisplay,
      config: EGLConfig,
      win: EGLNativeWindowType,
      attrib_list: *const EGLint
   ) -> EGLSurface;

   pub fn eglMakeCurrent(
      display: EGLDisplay,
      draw: EGLSurface,
      read: EGLSurface,
      ctx: EGLContext
   ) -> EGLBoolean;

   pub fn eglQueryContext(
      display: EGLDisplay,
      ctx: EGLContext,
      attribute: EGLint,
      value: *mut EGLint
   ) -> EGLBoolean;

   pub fn eglSwapBuffers(
      display: EGLDisplay,
      surface: EGLSurface
   ) -> EGLBoolean;

   pub fn eglSwapInterval(
      display: EGLDisplay,
      interval: EGLint
   ) -> EGLBoolean;
}

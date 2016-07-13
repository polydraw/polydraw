#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{EGLNativeDisplayType, EGLNativeWindowType};

use std::mem;

pub use libc::{
   c_char, c_int, c_uint, c_void
};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

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

pub const EGL_CONTEXT_FLAGS_KHR:            EGLenum = 0x30FC;

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

pub const EGL_CONTEXT_OPENGL_DEBUG_BIT_KHR:               EGLint = 0x00000001;
pub const EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE_BIT_KHR:  EGLint = 0x00000002;
pub const EGL_CONTEXT_OPENGL_ROBUST_ACCESS_BIT_KHR:       EGLint = 0x00000004;

pub type __eglMustCastToProperFunctionPointerType =
//   Option<extern "C" fn() -> ()>;
   *const c_void;

static mut eglGetErrorPtr:                       FnPtr = NULL_PTR;
static mut eglBindAPIPtr:                        FnPtr = NULL_PTR;
static mut eglGetDisplayPtr:                     FnPtr = NULL_PTR;
static mut eglInitializePtr:                     FnPtr = NULL_PTR;
static mut eglGetConfigsPtr:                     FnPtr = NULL_PTR;
static mut eglChooseConfigPtr:                   FnPtr = NULL_PTR;
static mut eglGetConfigAttribPtr:                FnPtr = NULL_PTR;
static mut eglGetProcAddressPtr:                 FnPtr = NULL_PTR;
static mut eglCreateContextPtr:                  FnPtr = NULL_PTR;
static mut eglCreateWindowSurfacePtr:            FnPtr = NULL_PTR;
static mut eglMakeCurrentPtr:                    FnPtr = NULL_PTR;
static mut eglQueryContextPtr:                   FnPtr = NULL_PTR;
static mut eglSwapBuffersPtr:                    FnPtr = NULL_PTR;
static mut eglSwapIntervalPtr:                   FnPtr = NULL_PTR;

#[inline]
pub unsafe fn eglGetError() -> EGLint {
   mem::transmute::<_, extern "system" fn() -> EGLint>(eglGetErrorPtr)()
}

#[inline]
pub unsafe fn eglBindAPI(api: EGLenum) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLenum) -> EGLBoolean>(eglBindAPIPtr)(api)
}

#[inline]
pub unsafe fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
   mem::transmute::<_, extern "system" fn(EGLNativeDisplayType) -> EGLDisplay>(eglGetDisplayPtr)(display_id)
}

#[inline]
pub unsafe fn eglInitialize(display: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, *mut EGLint, *mut EGLint) -> EGLBoolean>(eglInitializePtr)(display, major, minor)
}

#[inline]
pub unsafe fn eglGetConfigs(display: EGLDisplay, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, *mut EGLConfig, EGLint, *mut EGLint) -> EGLBoolean>(eglGetConfigsPtr)(display, configs, config_size, num_config)
}

#[inline]
pub unsafe fn eglChooseConfig(display: EGLDisplay, attrib_list: *const EGLint, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, *const EGLint, *mut EGLConfig, EGLint, *mut EGLint) -> EGLBoolean>(eglChooseConfigPtr)(display, attrib_list, configs, config_size, num_config)
}

#[inline]
pub unsafe fn eglGetConfigAttrib(display: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *mut EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLConfig, EGLint, *mut EGLint) -> EGLBoolean>(eglGetConfigAttribPtr)(display, config, attribute, value)
}

#[inline]
pub unsafe fn eglGetProcAddress(procname: *const c_char) -> __eglMustCastToProperFunctionPointerType {
   mem::transmute::<_, extern "system" fn(*const c_char) -> __eglMustCastToProperFunctionPointerType>(eglGetProcAddressPtr)(procname)
}

#[inline]
pub unsafe fn eglCreateContext(display: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const EGLint) -> EGLContext {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLConfig, EGLContext, *const EGLint) -> EGLContext>(eglCreateContextPtr)(display, config, share_context, attrib_list)
}

#[inline]
pub unsafe fn eglCreateWindowSurface(display: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const EGLint) -> EGLSurface {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLConfig, EGLNativeWindowType, *const EGLint) -> EGLSurface>(eglCreateWindowSurfacePtr)(display, config, win, attrib_list)
}

#[inline]
pub unsafe fn eglMakeCurrent(display: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLSurface, EGLSurface, EGLContext) -> EGLBoolean>(eglMakeCurrentPtr)(display, draw, read, ctx)
}

#[inline]
pub unsafe fn eglQueryContext(display: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *mut EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLContext, EGLint, *mut EGLint) -> EGLBoolean>(eglQueryContextPtr)(display, ctx, attribute, value)
}

#[inline]
pub unsafe fn eglSwapBuffers(display: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLSurface) -> EGLBoolean>(eglSwapBuffersPtr)(display, surface)
}

#[inline]
pub unsafe fn eglSwapInterval(display: EGLDisplay, interval: EGLint) -> EGLBoolean {
   mem::transmute::<_, extern "system" fn(EGLDisplay, EGLint) -> EGLBoolean>(eglSwapIntervalPtr)(display, interval)
}

pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   eglGetErrorPtr = loader.load("eglGetError");
   eglBindAPIPtr = loader.load("eglBindAPI");
   eglGetDisplayPtr = loader.load("eglGetDisplay");
   eglInitializePtr = loader.load("eglInitialize");
   eglGetConfigsPtr = loader.load("eglGetConfigs");
   eglChooseConfigPtr = loader.load("eglChooseConfig");
   eglGetConfigAttribPtr = loader.load("eglGetConfigAttrib");
   eglGetProcAddressPtr = loader.load("eglGetProcAddress");
   eglCreateContextPtr = loader.load("eglCreateContext");
   eglCreateWindowSurfacePtr = loader.load("eglCreateWindowSurface");
   eglMakeCurrentPtr = loader.load("eglMakeCurrent");
   eglQueryContextPtr = loader.load("eglQueryContext");
   eglSwapBuffersPtr = loader.load("eglSwapBuffers");
   eglSwapIntervalPtr = loader.load("eglSwapInterval");

   true
}

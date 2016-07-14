#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

pub use libc::{
   c_char, c_uchar, c_int, c_void,
};

use super::GLXNativeWindowType;

pub use sys::gl::ffi::GLubyte;
pub use sys::x11::ffi::{XID, Display, XVisualInfo};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

pub type GLXDrawable = XID;
pub type GLXWindow = XID;
pub type Bool = c_int;

pub const GLX_WINDOW_BIT:                 c_int = 0x0001;
pub const GLX_PIXMAP_BIT:                 c_int = 0x0002;
pub const GLX_PBUFFER_BIT:                c_int = 0x0004;

pub const GLX_USE_GL:                     c_int = 0x0001;
pub const GLX_BUFFER_SIZE:                c_int = 0x0002;
pub const GLX_LEVEL:                      c_int = 0x0003;
pub const GLX_RGBA:                       c_int = 0x0004;
pub const GLX_DOUBLEBUFFER:               c_int = 0x0005;
pub const GLX_STEREO:                     c_int = 0x0006;
pub const GLX_AUX_BUFFERS:                c_int = 0x0007;
pub const GLX_RED_SIZE:                   c_int = 0x0008;
pub const GLX_GREEN_SIZE:                 c_int = 0x0009;
pub const GLX_BLUE_SIZE:                  c_int = 0x000a;
pub const GLX_ALPHA_SIZE:                 c_int = 0x000b;
pub const GLX_DEPTH_SIZE:                 c_int = 0x000c;
pub const GLX_STENCIL_SIZE:               c_int = 0x000d;
pub const GLX_ACCUM_RED_SIZE:             c_int = 0x000e;
pub const GLX_ACCUM_GREEN_SIZE:           c_int = 0x000f;
pub const GLX_ACCUM_BLUE_SIZE:            c_int = 0x0010;
pub const GLX_ACCUM_ALPHA_SIZE:           c_int = 0x0011;

pub const GLX_CONFIG_CAVEAT:              c_int = 0x0020;
pub const GLX_X_VISUAL_TYPE:              c_int = 0x0022;
pub const GLX_TRANSPARENT_TYPE:           c_int = 0x0023;
pub const GLX_TRANSPARENT_INDEX_VALUE:    c_int = 0x0024;
pub const GLX_TRANSPARENT_RED_VALUE:      c_int = 0x0025;
pub const GLX_TRANSPARENT_GREEN_VALUE:    c_int = 0x0026;
pub const GLX_TRANSPARENT_BLUE_VALUE:     c_int = 0x0027;
pub const GLX_TRANSPARENT_ALPHA_VALUE:    c_int = 0x0028;

pub const GLX_DRAWABLE_TYPE:              c_int = 0x8010;
pub const GLX_RENDER_TYPE:                c_int = 0x8011;
pub const GLX_X_RENDERABLE:               c_int = 0x8012;
pub const GLX_FBCONFIG_ID:                c_int = 0x8013;
pub const GLX_RGBA_TYPE:                  c_int = 0x8014;

pub const GLX_RGBA_BIT:                   c_int = 0x0001;
pub const GLX_COLOR_INDEX_BIT:            c_int = 0x0002;

pub const GLX_TRUE_COLOR:                 c_int = 0x8002;
pub const GLX_DIRECT_COLOR:               c_int = 0x8003;
pub const GLX_PSEUDO_COLOR:               c_int = 0x8004;
pub const GLX_STATIC_COLOR:               c_int = 0x8005;
pub const GLX_GRAY_SCALE:                 c_int = 0x8006;
pub const GLX_STATIC_GRAY:                c_int = 0x8007;
pub const GLX_TRANSPARENT_RGB:            c_int = 0x8008;
pub const GLX_TRANSPARENT_INDEX:          c_int = 0x8009;

pub enum GLXFBConfigRec {}
pub type GLXFBConfig = *mut GLXFBConfigRec;

pub enum GLXcontextRec {}
pub type GLXContext = *mut GLXcontextRec;

pub type __glXMustCastToProperFunctionPointerType =
//   Option<extern "C" fn(procname: *const GLubyte) -> ()>
   *const c_void;

static mut glXQueryVersionPtr:                                FnPtr = NULL_PTR;
static mut glXGetProcAddressPtr:                              FnPtr = NULL_PTR;
static mut glXGetVisualFromFBConfigPtr:                       FnPtr = NULL_PTR;
static mut glXChooseFBConfigPtr:                              FnPtr = NULL_PTR;
static mut glXGetFBConfigAttribPtr:                           FnPtr = NULL_PTR;
static mut glXGetCurrentContextPtr:                           FnPtr = NULL_PTR;
static mut glXCreateNewContextPtr:                            FnPtr = NULL_PTR;
static mut glXSwapBuffersPtr:                                 FnPtr = NULL_PTR;
static mut glXDestroyContextPtr:                              FnPtr = NULL_PTR;
static mut glXMakeCurrentPtr:                                 FnPtr = NULL_PTR;
static mut glXCreateWindowPtr:                                FnPtr = NULL_PTR;
static mut glXMakeContextCurrentPtr:                          FnPtr = NULL_PTR;

#[inline]
pub unsafe fn glXQueryVersion(display: *mut Display, major: *mut c_int, minor: *mut c_int) -> c_int {
   mem::transmute::<_, extern "system" fn(*mut Display, *mut c_int, *mut c_int) -> c_int>(glXQueryVersionPtr)(display, major, minor)
}

#[inline]
pub unsafe fn glXGetProcAddress(procname: *const c_char) -> __glXMustCastToProperFunctionPointerType {
   mem::transmute::<_, extern "system" fn(*const c_char) -> __glXMustCastToProperFunctionPointerType>(glXGetProcAddressPtr)(procname)
}

#[inline]
pub unsafe fn glXGetVisualFromFBConfig(display: *mut Display, config: GLXFBConfig) -> *mut XVisualInfo {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXFBConfig) -> *mut XVisualInfo>(glXGetVisualFromFBConfigPtr)(display, config)
}

#[inline]
pub unsafe fn glXChooseFBConfig(display: *mut Display, screen: c_int, attribList: *const c_int, nitems: *mut c_int) -> *mut GLXFBConfig {
   mem::transmute::<_, extern "system" fn(*mut Display, c_int, *const c_int, *mut c_int) -> *mut GLXFBConfig>(glXChooseFBConfigPtr)(display, screen, attribList, nitems)
}

#[inline]
pub unsafe fn glXGetFBConfigAttrib(display: *mut Display, config: GLXFBConfig, attribute: c_int, value: *mut c_int) -> c_int {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXFBConfig, c_int, *mut c_int) -> c_int>(glXGetFBConfigAttribPtr)(display, config, attribute, value)
}

#[inline]
pub unsafe fn glXGetCurrentContext() -> GLXContext {
   mem::transmute::<_, extern "system" fn() -> GLXContext>(glXGetCurrentContextPtr)()
}

#[inline]
pub unsafe fn glXCreateNewContext(display: *mut Display, config: GLXFBConfig, renderType: c_int, shareList: GLXContext, direct: c_int) -> GLXContext {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXFBConfig, c_int, GLXContext, c_int) -> GLXContext>(glXCreateNewContextPtr)(display, config, renderType, shareList, direct)
}

#[inline]
pub unsafe fn glXSwapBuffers(display: *mut Display, drawable: GLXDrawable) {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXDrawable) -> ()>(glXSwapBuffersPtr)(display, drawable)
}

#[inline]
pub unsafe fn glXDestroyContext(display: *mut Display, ctx: GLXContext) {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXContext) -> ()>(glXDestroyContextPtr)(display, ctx)
}

#[inline]
pub unsafe fn glXMakeCurrent(display: *mut Display, drawable: GLXDrawable, ctx: GLXContext) -> c_int {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXDrawable, GLXContext) -> c_int>(glXMakeCurrentPtr)(display, drawable, ctx)
}

#[inline]
pub unsafe fn glXCreateWindow(display: *mut Display, config: GLXFBConfig, window: GLXNativeWindowType, attrib_list: *const c_int) -> GLXWindow {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXFBConfig, GLXNativeWindowType, *const c_int) -> GLXWindow>(glXCreateWindowPtr)(display, config, window, attrib_list)
}

#[inline]
pub unsafe fn glXMakeContextCurrent(display: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext) -> Bool {
   mem::transmute::<_, extern "system" fn(*mut Display, GLXDrawable, GLXDrawable, GLXContext) -> Bool>(glXMakeContextCurrentPtr)(display, draw, read, ctx)
}

pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   glXQueryVersionPtr = loader.load("glXQueryVersion");
   glXGetProcAddressPtr = loader.load("glXGetProcAddress");
   glXGetVisualFromFBConfigPtr = loader.load("glXGetVisualFromFBConfig");
   glXChooseFBConfigPtr = loader.load("glXChooseFBConfig");
   glXGetFBConfigAttribPtr = loader.load("glXGetFBConfigAttrib");
   glXGetCurrentContextPtr = loader.load("glXGetCurrentContext");
   glXCreateNewContextPtr = loader.load("glXCreateNewContext");
   glXSwapBuffersPtr = loader.load("glXSwapBuffers");
   glXDestroyContextPtr = loader.load("glXDestroyContext");
   glXMakeCurrentPtr = loader.load("glXMakeCurrent");
   glXCreateWindowPtr = loader.load("glXCreateWindow");
   glXMakeContextCurrentPtr = loader.load("glXMakeContextCurrent");

   true
}

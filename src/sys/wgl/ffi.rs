#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::mem;

pub use libc::{
   c_char, c_int, c_uint, c_void
};
pub use super::super::win32::ffi::{
   WORD, DWORD, BYTE, BOOL, HANDLE, TRUE, HDC
};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

pub type HGLRC = HANDLE;

pub const PFD_TYPE_RGBA:          BYTE = 0;

pub const PFD_MAIN_PLANE:         BYTE = 0;

pub const PFD_DOUBLEBUFFER:      DWORD = 0x00000001;
pub const PFD_DRAW_TO_WINDOW:    DWORD = 0x00000004;
pub const PFD_SUPPORT_OPENGL:    DWORD = 0x00000020;

#[repr(C)]
#[derive(Copy)]
pub struct PIXELFORMATDESCRIPTOR {
   pub nSize: WORD,
   pub nVersion: WORD,
   pub dwFlags: DWORD,
   pub iPixelType: BYTE,
   pub cColorBits: BYTE,
   pub cRedBits: BYTE,
   pub cRedShift: BYTE,
   pub cGreenBits: BYTE,
   pub cGreenShift: BYTE,
   pub cBlueBits: BYTE,
   pub cBlueShift: BYTE,
   pub cAlphaBits: BYTE,
   pub cAlphaShift: BYTE,
   pub cAccumBits: BYTE,
   pub cAccumRedBits: BYTE,
   pub cAccumGreenBits: BYTE,
   pub cAccumBlueBits: BYTE,
   pub cAccumAlphaBits: BYTE,
   pub cDepthBits: BYTE,
   pub cStencilBits: BYTE,
   pub cAuxBuffers: BYTE,
   pub iLayerType: BYTE,
   pub bReserved: BYTE,
   pub dwLayerMask: DWORD,
   pub dwVisibleMask: DWORD,
   pub dwDamageMask: DWORD,
}
impl Clone for PIXELFORMATDESCRIPTOR {
   fn clone(&self) -> Self { *self }
}
impl Default for PIXELFORMATDESCRIPTOR {
   fn default() -> Self { unsafe { mem::zeroed() } }
}

static mut wglSwapIntervalEXTPtr:            FnPtr = NULL_PTR;
static mut wglCreateContextPtr:              FnPtr = NULL_PTR;
static mut wglMakeCurrentPtr:                FnPtr = NULL_PTR;
static mut wglDeleteContextPtr:              FnPtr = NULL_PTR;
static mut wglGetCurrentContextPtr:          FnPtr = NULL_PTR;
static mut wglGetProcAddressPtr:             FnPtr = NULL_PTR;

#[inline]
pub unsafe fn wglSwapIntervalEXT(interval: c_int) -> BOOL {
   mem::transmute::<_, extern "system" fn(c_int) -> BOOL>(wglSwapIntervalEXTPtr)(interval)
}

#[inline]
pub unsafe fn wglCreateContext(hdc: HDC) -> HGLRC {
   mem::transmute::<_, extern "system" fn(HDC) -> HGLRC>(wglCreateContextPtr)(hdc)
}

#[inline]
pub unsafe fn wglMakeCurrent(hdc: HDC, rc: HGLRC) -> BOOL {
   mem::transmute::<_, extern "system" fn(HDC, HGLRC) -> BOOL>(wglMakeCurrentPtr)(hdc, rc)
}

#[inline]
pub unsafe fn wglDeleteContext(rc: HGLRC) -> BOOL {
   mem::transmute::<_, extern "system" fn(HGLRC) -> BOOL>(wglDeleteContextPtr)(rc)
}

#[inline]
pub unsafe fn wglGetCurrentContext() -> HGLRC {
   mem::transmute::<_, extern "system" fn() -> HGLRC>(wglGetCurrentContextPtr)()
}

#[inline]
pub unsafe fn wglGetProcAddress(name: *const c_char) -> *const c_void {
   mem::transmute::<_, extern "system" fn(*const c_char) -> *const c_void>(wglGetProcAddressPtr)(name)
}

pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   wglGetProcAddressPtr = loader.load("wglGetProcAddress");
   wglCreateContextPtr = loader.load("wglCreateContext");
   wglMakeCurrentPtr = loader.load("wglMakeCurrent");
   wglDeleteContextPtr = loader.load("wglDeleteContext");
   wglGetCurrentContextPtr = loader.load("wglGetCurrentContext");

   true
}

pub unsafe fn load_extra_functions<T: FnPtrLoader>(loader: &T) -> bool {
   wglSwapIntervalEXTPtr = loader.load("wglSwapIntervalEXT");

   true
}

#[link(name="gdi32")]
extern "C" {
   pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;

   pub fn DescribePixelFormat(hdc: HDC, iPixelFormat: c_int, nBytes: c_uint, ppfd: *mut PIXELFORMATDESCRIPTOR) -> c_int;

   pub fn SetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;

   pub fn SwapBuffers(rc: HDC) -> BOOL;
}

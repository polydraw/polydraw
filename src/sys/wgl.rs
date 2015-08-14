#![cfg(target_os = "windows")]

pub mod ffi {
   #![allow(non_snake_case)]

   use std::mem;

   pub use libc::{
      c_char, c_int, c_uint, c_void
   };
   pub use libc::types::os::arch::extra::{
      WORD, DWORD, BYTE, BOOL, HANDLE
   };
   pub use libc::consts::os::extra::{
      TRUE
   };
   pub use super::super::win::ffi::{
      HDC
   };

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

   #[link(name="gdi32")]
   extern "C" {
      pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;

      pub fn DescribePixelFormat(hdc: HDC, iPixelFormat: c_int, nBytes: c_uint, ppfd: *mut PIXELFORMATDESCRIPTOR) -> c_int;

      pub fn SetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;

      pub fn SwapBuffers(rc: HDC) -> BOOL;
   }

   #[link(name="opengl32")]
   extern "C" {
      pub fn wglCreateContext(hdc: HDC) -> HGLRC;

      pub fn wglMakeCurrent(hdc: HDC, rc: HGLRC) -> BOOL;

      pub fn wglDeleteContext(rc: HGLRC) -> BOOL;

      pub fn wglGetCurrentContext() -> HGLRC;

      pub fn wglGetProcAddress(name: *const c_char) -> *const c_void;
   }
}

use std::io;
use std::mem;
use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};
use super::utils::fn_ptr::{FnPtrLoader, FnPtr};

pub fn init_pixel_format(
   hdc: ffi::HDC,
) -> Result<(), RuntimeError> {
   let mut pfd = ffi::PIXELFORMATDESCRIPTOR {
      nSize: mem::size_of::<ffi::PIXELFORMATDESCRIPTOR>() as ffi::WORD,
      nVersion: 1,
      dwFlags: ffi::PFD_DRAW_TO_WINDOW | ffi::PFD_SUPPORT_OPENGL | ffi::PFD_DOUBLEBUFFER,
      iPixelType: ffi::PFD_TYPE_RGBA,
      cColorBits: 24,
      cRedBits: 0, cRedShift: 0, cGreenBits: 0, cGreenShift: 0, cBlueBits: 0, cBlueShift: 0,
      cAlphaBits: 0, cAlphaShift: 0, cAccumBits: 0,
      cAccumRedBits: 0, cAccumGreenBits: 0, cAccumBlueBits: 0, cAccumAlphaBits: 0,
      cDepthBits: 0,
      cStencilBits: 0,
      cAuxBuffers: 0,
      iLayerType: ffi::PFD_MAIN_PLANE,
      bReserved: 0,
      dwLayerMask: 0, dwVisibleMask: 0, dwDamageMask: 0
   };

   let pixel_format = unsafe { ffi::ChoosePixelFormat(hdc, &pfd) };

   if pixel_format == 0 {
      return Err(RuntimeError::new(
         ErrorKind::WGL,
         "Choosing pixel format failed".to_string()
      ));
   }

   unsafe { ffi::DescribePixelFormat(hdc, pixel_format, mem::size_of::<ffi::PIXELFORMATDESCRIPTOR>() as ffi::c_uint, &mut pfd) };

   let result = unsafe { ffi::SetPixelFormat(hdc, pixel_format, &pfd) };

   if result != ffi::TRUE {
      return Err(RuntimeError::new(
         ErrorKind::WGL,
         "Setting pixel format failed".to_string()
      ));
   }

   Ok(())
}

pub struct Context {
   pub rc: ffi::HGLRC
}

impl Context {
   pub fn create(hdc: ffi::HDC) -> Result<Self, RuntimeError> {
      let rc = unsafe { ffi::wglCreateContext(hdc) };

      if rc == ptr::null_mut() {
         return Err(RuntimeError::new(
            ErrorKind::WGL,
            format!("Create WGL context failed: {}", io::Error::last_os_error())
         ));
      }

      let result = unsafe { ffi::wglMakeCurrent(hdc, rc) };

      if result != ffi::TRUE {
         return Err(RuntimeError::new(
            ErrorKind::WGL,
            "wglMakeCurrent failed".to_string()
         ));
      }

      Ok(Context {
         rc: rc,
      })
   }

   pub fn current() -> Result<Self, RuntimeError> {
      let rc = unsafe { ffi::wglGetCurrentContext() };

      if rc == ptr::null_mut() {
         return Err(RuntimeError::new(
            ErrorKind::WGL,
            "Getting current WGL context failed".to_string()
         ));
      }

      Ok(Context {
         rc: rc,
      })
   }
}

impl Drop for Context {
   fn drop (&mut self) {
      unsafe {
         ffi::wglDeleteContext(self.rc);
      }
   }
}

pub fn swap_buffers(hdc: ffi::HDC) {
   unsafe { ffi::SwapBuffers(hdc) };
}

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
         ffi::wglGetProcAddress(cname)
      };

      addr
   }
}

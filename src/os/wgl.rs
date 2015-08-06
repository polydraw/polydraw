#![cfg(target_os = "windows")]

pub mod ffi {
   #![allow(non_snake_case)]

   use std::mem;

   pub use libc::{
      c_char, c_int, c_uint, c_void
   };
   pub use libc::types::os::arch::extra::{
      WORD, DWORD, BYTE, BOOL
   };
   pub use libc::consts::os::extra::{
      TRUE
   };
   pub use os::win::ffi::{
      HDC
   };

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

   #[link(name="opengl32")]
   extern "C" {
      pub fn wglChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;

      pub fn wglSetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;
   }
}

use std::mem;

use ::error::{RuntimeError, ErrorKind};

pub fn init_pixel_format(
   hdc: ffi::HDC,
) -> Result<(), RuntimeError> {
   let pfd = ffi::PIXELFORMATDESCRIPTOR {
      nSize: mem::size_of::<ffi::PIXELFORMATDESCRIPTOR>() as ffi::WORD,
      nVersion: 1,
      dwFlags: ffi::PFD_DRAW_TO_WINDOW | ffi::PFD_SUPPORT_OPENGL | ffi::PFD_DOUBLEBUFFER,
      iPixelType: ffi::PFD_TYPE_RGBA,
      cColorBits: 32,
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


   let pixel_format = unsafe { ffi::wglChoosePixelFormat(hdc, &pfd) };
   let result = unsafe { ffi::wglSetPixelFormat(hdc, pixel_format, &pfd) };

   if result != ffi::TRUE {
      return Err(RuntimeError::new(
         ErrorKind::WGL,
         "wglSetPixelFormat failed".to_string()
      ));
   }

   Ok(())
}

#![cfg(target_os = "windows")]

pub mod ffi {
   #![allow(non_snake_case)]

   pub use libc::types::os::arch::extra::{
      WORD, DWORD, BYTE
   };
   use std::mem;

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
}

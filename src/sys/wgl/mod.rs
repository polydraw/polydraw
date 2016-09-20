#![cfg(target_os = "windows")]

pub mod ffi;

use std::io;
use std::mem;
use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind, VoidResult};
use super::utils::fn_ptr::{FnPtrLoader, FnPtr};

use super::win32;

#[inline]
pub fn initialize<T: FnPtrLoader>(loader: &T) {
   unsafe {
      ffi::load_functions(loader)
   };
}

#[inline]
pub fn load_extra_functions<T: FnPtrLoader>(loader: &T) {
   unsafe {
      ffi::load_extra_functions(loader)
   };
}

pub fn init_pixel_format(
   device_context: &win32::DeviceContext,
) -> VoidResult {
   let mut pfd = ffi::PIXELFORMATDESCRIPTOR {
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

   let pixel_format = unsafe {
      ffi::ChoosePixelFormat(device_context.hdc, &pfd)
   };

   if pixel_format == 0 {
      return Err(RuntimeError::new(
         ErrorKind::WGL,
         "Choosing pixel format failed".to_string()
      ));
   }

   unsafe {
      ffi::DescribePixelFormat(
         device_context.hdc,
         pixel_format,
         mem::size_of::<ffi::PIXELFORMATDESCRIPTOR>() as ffi::c_uint, &mut pfd
      )
   };

   let result = unsafe {
      ffi::SetPixelFormat(device_context.hdc, pixel_format, &pfd)
   };

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
   pub fn create(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      let rc = unsafe {
         ffi::wglCreateContext(device_context.hdc)
      };

      if rc == ptr::null_mut() {
         return Err(RuntimeError::new(
            ErrorKind::WGL,
            format!("Create WGL context failed: {}", io::Error::last_os_error())
         ));
      }

      let result = unsafe {
         ffi::wglMakeCurrent(device_context.hdc, rc)
      };

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

   #[inline]
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

#[inline]
pub fn swap_buffers(device_context: &win32::DeviceContext) {
   unsafe {
      ffi::SwapBuffers(device_context.hdc)
   };
}

pub fn swap_interval(interval: ffi::c_int) -> VoidResult {
   let result = unsafe {
      ffi::wglSwapIntervalEXT(interval)
   };

   if result != ffi::TRUE {
      return Err(RuntimeError::new(
         ErrorKind::WGL,
         "wglSwapIntervalEXT failed".to_string()
      ));
   }

   Ok(())
}

pub struct Loader {
   library_loader: Box<FnPtrLoader>,
}

impl Loader {
   pub fn new(library_loader: Box<FnPtrLoader>) -> Self {
      Loader {
         library_loader: library_loader
      }
   }
}

impl FnPtrLoader for Loader {
   fn load(&self, name: &str) -> FnPtr {
      let lib_addr = self.library_loader.load(name);

      if !lib_addr.is_null() {
         return lib_addr;
      }

      let cname = CString::new(name).unwrap();

      let addr = unsafe {
         ffi::wglGetProcAddress(cname.as_ptr())
      };

      addr
   }
}

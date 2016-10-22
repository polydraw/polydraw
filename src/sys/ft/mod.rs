pub mod ffi;

use std::ptr;

use super::DynLibrary;
use super::utils::fn_ptr::FnPtrLibrary;

pub struct FreeType {
   pub dyn_lib: DynLibrary,
   pub ft_lib: ffi::FT_Library,
}

impl FreeType {
   pub fn new() -> Self {
      let dyn_lib = FreeType::load_library();

      unsafe {
         ffi::load_functions(&dyn_lib);

         let mut ft_lib: ffi::FT_Library = ptr::null_mut();

         ffi::FT_Init_FreeType(&mut ft_lib);

         FreeType {
            ft_lib: ft_lib,
            dyn_lib: dyn_lib,
         }
      }
   }

   #[inline]
   #[cfg(target_os = "linux")]
   pub fn load_library() -> DynLibrary {
      DynLibrary::open("libfreetype.so.6").unwrap()
   }

   #[inline]
   #[cfg(target_os = "windows")]
   pub fn load_library() -> DynLibrary {
      DynLibrary::open("freetype.dll").unwrap()
   }
}

impl Drop for FreeType {
   fn drop (&mut self) {
      unsafe {
         ffi::FT_Done_FreeType(self.ft_lib);
      }
   }
}

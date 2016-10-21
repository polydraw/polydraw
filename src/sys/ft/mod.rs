pub mod ffi;

use std::ptr;

use super::DynLibrary;

pub struct FreeType {
   pub dyn_lib: DynLibrary,
   pub ft_lib: ffi::FT_Library,
}

impl FreeType {
   pub fn new(dyn_lib: DynLibrary) -> Self {
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
}

impl Drop for FreeType {
   fn drop (&mut self) {
      unsafe {
         ffi::FT_Done_FreeType(self.ft_lib);
      }
   }
}

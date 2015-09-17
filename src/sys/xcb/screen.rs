use super::ffi;

pub struct Screen {
   pub ptr: *mut ffi::xcb_screen_t
}

impl Screen {
   pub fn new(screen_ptr: *mut ffi::xcb_screen_t) -> Self {
      Screen {
         ptr: screen_ptr,
      }
   }

   pub fn root(&self) -> ffi::xcb_window_t {
      unsafe {
         (*self.ptr).root
      }
   }

   pub fn width_in_pixels(&self) -> u32 {
      unsafe {
         (*self.ptr).width_in_pixels as u32
      }
   }

   pub fn height_in_pixels(&self) -> u32 {
      unsafe {
         (*self.ptr).height_in_pixels as u32
      }
   }

   pub fn root_visual(&self) -> ffi::xcb_visualid_t {
      unsafe {
         (*self.ptr).root_visual
      }
   }
}

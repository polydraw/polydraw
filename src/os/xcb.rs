pub mod ffi {
   #![allow(non_camel_case_types)]

   pub enum xcb_connection_t { }

   #[link(name="xcb")]
   extern "C" {
   }
}

pub struct Connection {
   pub connection_ptr: *mut ffi::xcb_connection_t
}

impl Connection {
   pub fn new(connection_ptr: *mut ffi::xcb_connection_t) -> Self {
      Connection {
         connection_ptr: connection_ptr,
      }
   }
}

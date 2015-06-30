use libc::{c_char};

pub enum Display { }

#[link(name="X11")]
extern "C" {
   pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
}

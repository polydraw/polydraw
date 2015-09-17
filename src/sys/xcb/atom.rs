use super::ffi;

pub struct Atom {
   pub xcb_atom: ffi::xcb_atom_t
}

pub struct InternAtomCookie {
   pub xcb_cookie: ffi::xcb_intern_atom_cookie_t
}

pub struct InternAtomReply {
   pub xcb_reply: *mut ffi::xcb_intern_atom_reply_t
}

impl InternAtomReply {
   pub fn atom(&self) -> Atom {
      let xcb_atom = unsafe { (*(self.xcb_reply)).atom };
      Atom {
         xcb_atom: xcb_atom
      }
   }
}

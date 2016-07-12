use libc::c_void;

pub type FnPtr = *const c_void;

pub const NULL_PTR: FnPtr = 0 as FnPtr;

pub trait FnPtrLoader {
   fn load(&self, &str) -> FnPtr;
}

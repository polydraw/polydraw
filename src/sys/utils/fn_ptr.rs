use libc::c_void;

pub type FnPtr = *const c_void;

pub const NULL_PTR: FnPtr = 0 as FnPtr;

pub trait FnPtrLoader {
   fn get_proc_addr(&self, &str) -> FnPtr;
}

#[macro_export]
macro_rules! loadfn {
   ( $loader:ident, $name:expr ) => {
      {
         let fn_ptr = $loader.get_proc_addr($name);
         if fn_ptr == ::std::ptr::null() {
            return false;
         }
         fn_ptr
      }
   }
}

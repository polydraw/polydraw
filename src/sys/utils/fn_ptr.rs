use libc::c_void;

pub type FnPtr = *const c_void;

pub const NULL_PTR: FnPtr = 0 as FnPtr;

pub trait FnPtrLoader {
   fn load(&self, &str) -> FnPtr;

   #[inline]
   fn loadlist(&self, names: &[&str]) -> FnPtr {
      for name in names {
         let fn_ptr = self.load(name);
         if fn_ptr != NULL_PTR {
            return fn_ptr;
         }
      }

      NULL_PTR
   }
}

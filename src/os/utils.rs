macro_rules! field {
   ($that:ident, $field:ident) => {
      unsafe { (*$that.ptr).$field }
   };
}

#[macro_export]
macro_rules! getter {
   ($name:ident, $restype:ty) => {
      pub fn $name(&self) -> $restype {
         field!(self, $name) as $restype
      }
   }
}

use std::fmt;
use std::ptr;
use std::any::TypeId;

pub type VoidPtr = *mut ();

pub type ValuePtrList = Vec<ValuePtr>;


#[derive(Debug, Clone)]
pub struct ValuePtr {
   pub data: VoidPtr,
   pub type_id: TypeId,
}

impl ValuePtr {
   pub fn new<T: 'static>(value: T) -> Self where T: fmt::Debug {
      ValuePtr {
         data: Box::into_raw(Box::new(value)) as *mut (),
         type_id: TypeId::of::<T>(),
      }
   }

   pub fn null() -> Self {
      ValuePtr {
         data: ptr::null_mut(),
         type_id: TypeId::of::<()>(),
      }
   }
}


macro_rules! value_ptr_as_ref {
   ($ptr:expr, $ty:ty) => ({
      unsafe { ::std::mem::transmute::<VoidPtr, &mut $ty>($ptr.data) }
   })
}


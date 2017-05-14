use std::any::TypeId;
use std::fmt;


pub type VoidPtr = *mut ();

pub type CloneFn = fn(&Variant) -> Variant;

pub type DropFn = fn(&mut Variant);

pub type DebugFn = fn(&Variant, &mut fmt::Formatter) -> fmt::Result;

pub type VariantVec = Vec<Variant>;


pub struct VType {
   pub type_id: TypeId,
   pub vtable: VTable,
}


pub struct VTable {
   pub clone: CloneFn,
   pub drop: DropFn,
   pub debug: DebugFn,
}

pub struct Variant {
   pub data: VoidPtr,
   pub vtype: *const VType,
}

impl Variant {
   pub fn new<T: 'static>(value: T, vtype: &VType) -> Self {
      Variant {
         data: Box::into_raw(Box::new(value)) as VoidPtr,
         vtype: vtype,
      }
   }

   pub fn null() -> Self {
      Variant {
         data: ::std::ptr::null_mut(),
         vtype: ::std::ptr::null(),
      }
   }

   pub fn as_ref<T: 'static>(&self) -> &T {
      unsafe {
         ::std::mem::transmute::<VoidPtr, &T>(self.data)
      }
   }

   pub fn type_id(&self) -> &TypeId {
      unsafe {
         &(*self.vtype).type_id
      }
   }

   pub fn as_ref_checked<T: 'static>(&self) -> Option<&T> {
      unsafe {
         if TypeId::of::<T>() == (*self.vtype).type_id {
            Some(
               self.as_ref()
            )
         } else {
            None
         }
      }
   }
}

impl Clone for Variant {
   fn clone(&self) -> Self {
      unsafe {
         ((*self.vtype).vtable.clone)(self)
      }
   }
}


impl Drop for Variant {
   fn drop(&mut self) {
      unsafe {
         ((*self.vtype).vtable.drop)(self)
      }
   }
}


impl fmt::Debug for Variant {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      unsafe {
         ((*self.vtype).vtable.debug)(self, f)
      }
   }
}


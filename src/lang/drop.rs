use std::any::TypeId;
use std::collections::HashMap;
use std::ptr;

use super::super::data::{IntPoint, FloatPoint, Empty};
use super::value_ptr::{ValuePtr, ValuePtrList};
use super::compiler::FnRef;


pub type DropFunc = fn(&ValuePtr, &DropRegistry);

pub struct DropRegistry {
   map: HashMap<TypeId, DropFunc>,
}

impl DropRegistry {
   pub fn new() -> Self {
      DropRegistry {
         map: HashMap::new(),
      }
   }

   pub fn insert(&mut self, type_id: TypeId, func: DropFunc) {
      self.map.insert(type_id, func);
   }

   pub fn get(&self, type_id: &TypeId) -> Option<&DropFunc> {
      self.map.get(type_id)
   }
}

macro_rules! drop_func {
   ($drop_name:ident, $ty:ty) => {
      fn $drop_name(arg: &ValuePtr, _: &DropRegistry) {
         drop(
            unsafe { *Box::from_raw(arg.data as *mut $ty) }
         );
      }
   }
}

drop_func!(drop_i64, i64);

drop_func!(drop_f64, f64);

drop_func!(drop_bool, bool);

drop_func!(drop_string, String);

drop_func!(drop_empty, Empty);

drop_func!(drop_fnref, FnRef);

drop_func!(drop_int_point, IntPoint);

drop_func!(drop_float_point, FloatPoint);


fn drop_value_ptr_list(arg: &ValuePtr, drop_registry: &DropRegistry) {
   let list = unsafe { *Box::from_raw(arg.data as *mut ValuePtrList) };

   for value_ptr in list.iter() {
      drop_value_ptr(value_ptr, drop_registry);
   }

   drop(list);
}


pub fn create_drop_registry() -> DropRegistry {
   let mut drop_registry = DropRegistry::new();

   drop_registry.insert(TypeId::of::<i64>(), drop_i64);
   drop_registry.insert(TypeId::of::<f64>(), drop_f64);
   drop_registry.insert(TypeId::of::<bool>(), drop_bool);
   drop_registry.insert(TypeId::of::<String>(), drop_string);
   drop_registry.insert(TypeId::of::<Empty>(), drop_empty);
   drop_registry.insert(TypeId::of::<FnRef>(), drop_fnref);
   drop_registry.insert(TypeId::of::<ValuePtrList>(), drop_value_ptr_list);
   drop_registry.insert(TypeId::of::<IntPoint>(), drop_int_point);
   drop_registry.insert(TypeId::of::<FloatPoint>(), drop_float_point);

   drop_registry
}


pub fn drop_value_ptr(
   value_ptr: &ValuePtr,
   drop_registry: &DropRegistry
) {
   drop_registry.get(&value_ptr.type_id).unwrap()(value_ptr, drop_registry)
}


pub fn drop_value_ptr_vec(
   list: &Vec<ValuePtr>,
   drop_registry: &DropRegistry
) {
   for value_ptr in list.iter() {
      if value_ptr.data != ptr::null_mut() {
         drop_value_ptr(value_ptr, drop_registry);
      }
   }
}


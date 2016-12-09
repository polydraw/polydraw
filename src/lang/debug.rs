use std::any::TypeId;
use std::collections::HashMap;

use super::super::data::{IntPoint, FloatPoint, Empty};
use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::compiler::FnRef;


pub type DebugFunc = fn(&ValuePtr, &DebugRegistry) -> String;

pub struct DebugRegistry {
   map: HashMap<TypeId, DebugFunc>,
}

impl DebugRegistry {
   pub fn new() -> Self {
      DebugRegistry {
         map: HashMap::new(),
      }
   }

   pub fn insert(&mut self, type_id: TypeId, func: DebugFunc) {
      self.map.insert(type_id, func);
   }

   pub fn get(&self, type_id: &TypeId) -> Option<&DebugFunc> {
      self.map.get(type_id)
   }
}


macro_rules! debug_func {
   ($debug_name:ident, $ty:ty) => {
      fn $debug_name(arg: &ValuePtr, _: &DebugRegistry) -> String {
         format!("{:?}", value_ptr_as_ref!(arg, $ty))
      }
   }
}

debug_func!(debug_i64, i64);

debug_func!(debug_f64, f64);

debug_func!(debug_bool, bool);

debug_func!(debug_string, String);

debug_func!(debug_empty, Empty);

debug_func!(debug_fnref, FnRef);

debug_func!(debug_int_point, IntPoint);

debug_func!(debug_float_point, FloatPoint);


fn debug_value_ptr_list(arg: &ValuePtr, debug_registry: &DebugRegistry) -> String {
   let list = value_ptr_as_ref!(arg, ValuePtrList);

   let mut result = String::from("[");

   if list.len() > 0 {
      result.push_str(
         &debug_value_ptr(&list[0], debug_registry)
      );

      for value_ptr in list[1..].iter() {
         result.push(' ');

         result.push_str(
            &debug_value_ptr(value_ptr, debug_registry)
         );
      }
   }

   result.push(']');

   result
}


pub fn create_debug_registry() -> DebugRegistry {
   let mut debug_registry = DebugRegistry::new();

   debug_registry.insert(TypeId::of::<i64>(), debug_i64);
   debug_registry.insert(TypeId::of::<f64>(), debug_f64);
   debug_registry.insert(TypeId::of::<bool>(), debug_bool);
   debug_registry.insert(TypeId::of::<String>(), debug_string);
   debug_registry.insert(TypeId::of::<Empty>(), debug_empty);
   debug_registry.insert(TypeId::of::<FnRef>(), debug_fnref);
   debug_registry.insert(TypeId::of::<ValuePtrList>(), debug_value_ptr_list);
   debug_registry.insert(TypeId::of::<IntPoint>(), debug_int_point);
   debug_registry.insert(TypeId::of::<FloatPoint>(), debug_float_point);

   debug_registry
}


pub fn debug_value_ptr(
   value_ptr: &ValuePtr,
   debug_registry: &DebugRegistry
) -> String {
   debug_registry.get(&value_ptr.type_id).unwrap()(value_ptr, debug_registry)
}


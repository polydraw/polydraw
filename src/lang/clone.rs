use std::any::TypeId;
use std::collections::HashMap;

use sys::ft::Face;
use draw::RGB;
use devel::Poly;
use data::{IntPoint, FloatPoint, Empty};

use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::compiler::FnRef;


pub type CloneFunc = fn(&ValuePtr, &CloneRegistry) -> ValuePtr;


pub struct CloneRegistry {
   map: HashMap<TypeId, CloneFunc>,
}

impl CloneRegistry {
   pub fn new() -> Self {
      CloneRegistry {
         map: HashMap::new(),
      }
   }

   pub fn insert(&mut self, type_id: TypeId, func: CloneFunc) {
      self.map.insert(type_id, func);
   }

   pub fn get(&self, type_id: &TypeId) -> Option<&CloneFunc> {
      self.map.get(type_id)
   }
}


macro_rules! clone_func {
   ($clone_name:ident, $ty:ty) => {
      fn $clone_name(arg: &ValuePtr, _: &CloneRegistry) -> ValuePtr {
         ValuePtr::new(
            value_ptr_as_ref!(arg, $ty).clone()
         )
      }
   }
}

clone_func!(clone_i64, i64);

clone_func!(clone_f64, f64);

clone_func!(clone_bool, bool);

clone_func!(clone_string, String);

clone_func!(clone_empty, Empty);

clone_func!(clone_fnref, FnRef);

clone_func!(clone_int_point, IntPoint);

clone_func!(clone_float_point, FloatPoint);

clone_func!(clone_rgb, RGB);

clone_func!(clone_poly, Poly);

clone_func!(clone_face, Face);


pub fn clone_value_ptr_list(arg: &ValuePtr, clone_registry: &CloneRegistry) -> ValuePtr {
   let mut list = Vec::new();

   let arg_list = value_ptr_as_ref!(arg, ValuePtrList);

   for value_ptr in arg_list.iter() {
      list.push(
         clone_value_ptr(value_ptr, clone_registry)
      );
   }

   ValuePtr::new(list)
}


pub fn create_clone_registry() -> CloneRegistry {
   let mut clone_registry = CloneRegistry::new();

   clone_registry.insert(TypeId::of::<i64>(), clone_i64);
   clone_registry.insert(TypeId::of::<f64>(), clone_f64);
   clone_registry.insert(TypeId::of::<bool>(), clone_bool);
   clone_registry.insert(TypeId::of::<String>(), clone_string);
   clone_registry.insert(TypeId::of::<Empty>(), clone_empty);
   clone_registry.insert(TypeId::of::<FnRef>(), clone_fnref);
   clone_registry.insert(TypeId::of::<ValuePtrList>(), clone_value_ptr_list);
   clone_registry.insert(TypeId::of::<IntPoint>(), clone_int_point);
   clone_registry.insert(TypeId::of::<FloatPoint>(), clone_float_point);
   clone_registry.insert(TypeId::of::<RGB>(), clone_rgb);
   clone_registry.insert(TypeId::of::<Poly>(), clone_poly);
   clone_registry.insert(TypeId::of::<Face>(), clone_face);

   clone_registry
}


pub fn clone_value_ptr(
   value_ptr: &ValuePtr,
   clone_registry: &CloneRegistry
) -> ValuePtr {
   clone_registry.get(&value_ptr.type_id).unwrap()(value_ptr, clone_registry)
}

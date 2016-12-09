use std::any::TypeId;
use std::collections::HashMap;

use super::compiler::BuiltinIndices;
use super::value_ptr::ValuePtr;
use super::clone::CloneRegistry;
use super::execute::Executor;
use super::parser::FnIndex;
use super::number::{
   add_i64, add_f64, multiply_i64, multiply_f64, subtract_i64, subtract_f64,
   divide_i64, divide_f64,
};
use super::point::{
   point_f64_f64, point_f64_i64, point_i64_f64, point_i64_i64,
};
use super::list::{
   create_list, each, each_with_last, each_with_index,
};


type A2R1 = fn(&ValuePtr, &ValuePtr) -> ValuePtr;
type ALR1 = fn(Vec<&ValuePtr>, &CloneRegistry) -> ValuePtr;
type CALL = fn(Vec<&ValuePtr>, Executor) -> Vec<ValuePtr>;

type HMA2R1 = HashMap<(TypeId, TypeId), A2R1>;


pub enum TypeFnMap {
   HMA2R1(HMA2R1),
   CALL(CALL),
   ALR1(ALR1),
}


pub type FnList = Vec<TypeFnMap>;

macro_rules! wrap_operator {
   ($name:ident, $func:ident) => {
      pub fn $name(arg1: &ValuePtr, arg2: &ValuePtr) -> ValuePtr {
         ValuePtr::new(
            $func(
               unsafe { ::std::mem::transmute(arg1.data) },
               unsafe { ::std::mem::transmute(arg2.data) },
            )
         )
      }
   }
}


macro_rules! define_register_func {
   ($register_name:ident, $func_ty:ty, $map_tt:tt) => {
      fn $register_name(
         indices: &mut BuiltinIndices,
         fn_list: &mut FnList,
         name: &'static str,
         tyids: (TypeId, TypeId),
         func: $func_ty,
      ) {
         {
            if let Some(ref fn_index) = indices.get(name) {
               if let TypeFnMap::$map_tt(ref mut operator_map) = fn_list[fn_index.index] {
                  operator_map.insert(tyids, func);
               } else {
                  panic!("Incompatible types for built-in function '{}'", name);
               }
               return;
            }
         }

         {
            let index = fn_list.len();
            let fn_index = FnIndex::new(index, 1);
            indices.insert(name, fn_index);

            let mut operator_map = $map_tt::new();

            operator_map.insert(tyids, func);

            fn_list.push(TypeFnMap::$map_tt(operator_map));
         }
      }
   }
}


define_register_func!(register_a2r1, A2R1, HMA2R1);


fn register_list_arg_func(
   indices: &mut BuiltinIndices,
   fn_list: &mut FnList,
   name: &'static str,
   func: ALR1,
) {
   if indices.contains_key(name) {
      panic!("Cannot define a list arg function twice '{}'", name);
   }

   let index = fn_list.len();
   let fn_index = FnIndex::new(index, 1);
   indices.insert(name, fn_index);

   fn_list.push(TypeFnMap::ALR1(func));
}


fn register_call_func(
   indices: &mut BuiltinIndices,
   fn_list: &mut FnList,
   name: &'static str,
   func: CALL,
) {
   if indices.contains_key(name) {
      panic!("Cannot define a call function twice '{}'", name);
   }

   let index = fn_list.len();
   let fn_index = FnIndex::new(index, 1);
   indices.insert(name, fn_index);

   fn_list.push(TypeFnMap::CALL(func));
}


pub fn register_builtin_fns() -> (BuiltinIndices, FnList) {
   let mut indices = BuiltinIndices::new();

   let mut fn_list = FnList::new();

   let tyid_i64 = TypeId::of::<i64>();
   let tyid_f64 = TypeId::of::<f64>();

   register_a2r1(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_i64), add_i64);
   register_a2r1(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_f64), add_f64);

   register_a2r1(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_i64), multiply_i64);
   register_a2r1(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_f64), multiply_f64);

   register_a2r1(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_i64), subtract_i64);
   register_a2r1(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_f64), subtract_f64);

   register_a2r1(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_i64), divide_i64);
   register_a2r1(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_f64), divide_f64);

   register_a2r1(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_f64), point_f64_f64);
   register_a2r1(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_i64), point_f64_i64);
   register_a2r1(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_f64), point_i64_f64);
   register_a2r1(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_i64), point_i64_i64);

   register_list_arg_func(&mut indices, &mut fn_list, "list", create_list);

   register_call_func(&mut indices, &mut fn_list, "each", each);
   register_call_func(&mut indices, &mut fn_list, "each_with_last", each_with_last);
   register_call_func(&mut indices, &mut fn_list, "each_with_index", each_with_index);

   (indices, fn_list)
}


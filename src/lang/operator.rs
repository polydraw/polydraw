use std::any::TypeId;
use std::collections::HashMap;

use super::compiler::{BuiltinIndices, FnRef};
use super::value_ptr::{ValuePtr, ValuePtrList};
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
   create_list, each, each_with_last, each_with_index, list_2_arg,
};


type CALL = fn(Vec<&ValuePtr>, &Executor, &FnRef) -> Vec<ValuePtr>;

type HMA2R1 = HashMap<(TypeId, TypeId), CALL>;


pub enum TypeFnMap {
   HMA2R1(HMA2R1),
   CALL(CALL),
}


pub type FnList = Vec<TypeFnMap>;


macro_rules! vecval {
   ($value:expr) => {
      vec![ValuePtr::new($value)]
   }
}


macro_rules! wrap_operator {
   ($name:ident, $func:ident) => {
      pub fn $name(arguments: Vec<&ValuePtr>, _: &Executor, _: &FnRef) -> Vec<ValuePtr> {
         vecval!(
            $func(
               unsafe {
                  ::std::mem::transmute(arguments[0].data)
               },
               unsafe {
                  ::std::mem::transmute(arguments[1].data)
               },
            )
         )
      }
   }
}


macro_rules! define_register_func {
   ($register_name:ident, $tyids:ty, $map_tt:tt) => {
      fn $register_name(
         indices: &mut BuiltinIndices,
         fn_list: &mut FnList,
         name: &'static str,
         tyids: $tyids,
         func: CALL,
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


define_register_func!(register_2_arg, (TypeId, TypeId), HMA2R1);


fn register_n_arg(
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
   let tyid_list = TypeId::of::<ValuePtrList>();

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_i64), add_i64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_f64), add_f64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_list, tyid_i64), list_2_arg);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_list, tyid_f64), list_2_arg);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_i64), multiply_i64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_f64), multiply_f64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_list, tyid_i64), list_2_arg);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_list, tyid_f64), list_2_arg);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_i64), subtract_i64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_f64), subtract_f64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_list, tyid_i64), list_2_arg);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_list, tyid_f64), list_2_arg);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_i64), divide_i64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_f64), divide_f64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_list, tyid_i64), list_2_arg);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_list, tyid_f64), list_2_arg);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_f64), point_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_i64), point_f64_i64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_f64), point_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_i64), point_i64_i64);

   register_n_arg(&mut indices, &mut fn_list, "list", create_list);

   register_n_arg(&mut indices, &mut fn_list, "each", each);
   register_n_arg(&mut indices, &mut fn_list, "each_with_last", each_with_last);
   register_n_arg(&mut indices, &mut fn_list, "each_with_index", each_with_index);

   (indices, fn_list)
}


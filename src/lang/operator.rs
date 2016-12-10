use std::any::TypeId;
use std::collections::HashMap;

use super::super::data::{IntPoint, FloatPoint};

use super::compiler::{BuiltinIndices, FnRef};
use super::value_ptr::{ValuePtr, ValuePtrList};
use super::execute::Executor;
use super::parser::FnIndex;

use super::number::{
   add_i64_i64, add_f64_f64, add_i64_f64, add_f64_i64,
   multiply_i64_i64, multiply_f64_f64, multiply_i64_f64, multiply_f64_i64,
   subtract_i64_i64, subtract_f64_f64, subtract_i64_f64, subtract_f64_i64,
   divide_i64_i64, divide_f64_f64, divide_i64_f64, divide_f64_i64,
};

use super::point::{
   point_f64_f64, point_f64_i64, point_i64_f64, point_i64_i64,
   add_ipt_ipt, add_fpt_fpt, add_ipt_fpt, add_fpt_ipt,
   add_ipt_i64, add_fpt_f64, add_ipt_f64, add_fpt_i64,
   add_i64_ipt, add_f64_fpt, add_f64_ipt, add_i64_fpt,
   multiply_ipt_ipt, multiply_fpt_fpt, multiply_ipt_fpt, multiply_fpt_ipt,
   multiply_ipt_i64, multiply_fpt_f64, multiply_ipt_f64, multiply_fpt_i64,
   multiply_i64_ipt, multiply_f64_fpt, multiply_f64_ipt, multiply_i64_fpt,
   subtract_ipt_ipt, subtract_fpt_fpt, subtract_ipt_fpt, subtract_fpt_ipt,
   subtract_ipt_i64, subtract_fpt_f64, subtract_ipt_f64, subtract_fpt_i64,
   subtract_i64_ipt, subtract_f64_fpt, subtract_f64_ipt, subtract_i64_fpt,
   divide_ipt_ipt, divide_fpt_fpt, divide_ipt_fpt, divide_fpt_ipt,
   divide_ipt_i64, divide_fpt_f64, divide_ipt_f64, divide_fpt_i64,
   divide_i64_ipt, divide_f64_fpt, divide_f64_ipt, divide_i64_fpt,
   polar_f64_f64, polar_i64_i64, polar_f64_i64, polar_i64_f64,
   rotate_fpt_fpt_f64, rotate_ipt_fpt_f64, rotate_fpt_ipt_f64,
   rotate_fpt_fpt_i64, rotate_ipt_ipt_f64, rotate_ipt_fpt_i64,
   rotate_fpt_ipt_i64, rotate_ipt_ipt_i64,
};

use super::list::{
   list, each, each_with_last, each_with_index,
   list_lst_val, list_val_lst, list_lst_lst,
   list_lst_val_val, list_val_lst_val, list_val_val_lst,
   list_lst_lst_val, list_lst_val_lst, list_val_lst_lst, list_lst_lst_lst,
};


type CALL = fn(Vec<&ValuePtr>, &Executor, &FnRef) -> Vec<ValuePtr>;

type HMA2R1 = HashMap<(TypeId, TypeId), CALL>;

type HMA3R1 = HashMap<(TypeId, TypeId, TypeId), CALL>;


pub enum TypeFnMap {
   HMA2R1(HMA2R1),
   HMA3R1(HMA3R1),
   CALL(CALL),
}


pub type FnList = Vec<TypeFnMap>;


macro_rules! vecval {
   ($value:expr) => {
      vec![ValuePtr::new($value)]
   }
}


macro_rules! wrap_2_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(arguments: Vec<&ValuePtr>, _: &Executor, _: &FnRef) -> Vec<ValuePtr> {
         vecval!(
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
               unsafe { ::std::mem::transmute(arguments[1].data) },
            )
         )
      }
   }
}


macro_rules! wrap_3_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(arguments: Vec<&ValuePtr>, _: &Executor, _: &FnRef) -> Vec<ValuePtr> {
         vecval!(
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
               unsafe { ::std::mem::transmute(arguments[1].data) },
               unsafe { ::std::mem::transmute(arguments[2].data) },
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

define_register_func!(register_3_arg, (TypeId, TypeId, TypeId), HMA3R1);


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
   let tyid_lst = TypeId::of::<ValuePtrList>();
   let tyid_ipt = TypeId::of::<IntPoint>();
   let tyid_fpt = TypeId::of::<FloatPoint>();

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_i64), add_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_f64), add_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_f64), add_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_i64), add_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_ipt, tyid_ipt), add_ipt_ipt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_fpt), add_fpt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_ipt, tyid_fpt), add_ipt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_ipt), add_fpt_ipt);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_ipt, tyid_i64), add_ipt_i64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_f64), add_fpt_f64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_ipt, tyid_f64), add_ipt_f64);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_i64), add_fpt_i64);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_ipt), add_i64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_fpt), add_f64_fpt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_ipt), add_f64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_fpt), add_i64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_ipt), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_ipt, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_i64), multiply_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_f64), multiply_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_f64), multiply_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_i64), multiply_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_ipt, tyid_ipt), multiply_ipt_ipt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_fpt), multiply_fpt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_ipt, tyid_fpt), multiply_ipt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_ipt), multiply_fpt_ipt);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_ipt, tyid_i64), multiply_ipt_i64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_f64), multiply_fpt_f64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_ipt, tyid_f64), multiply_ipt_f64);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_i64), multiply_fpt_i64);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_ipt), multiply_i64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_fpt), multiply_f64_fpt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_ipt), multiply_f64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_fpt), multiply_i64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_ipt), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_ipt, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_i64), subtract_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_f64), subtract_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_f64), subtract_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_i64), subtract_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_ipt, tyid_ipt), subtract_ipt_ipt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_fpt), subtract_fpt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_ipt, tyid_fpt), subtract_ipt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_ipt), subtract_fpt_ipt);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_ipt, tyid_i64), subtract_ipt_i64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_f64), subtract_fpt_f64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_ipt, tyid_f64), subtract_ipt_f64);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_i64), subtract_fpt_i64);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_ipt), subtract_i64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_fpt), subtract_f64_fpt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_ipt), subtract_f64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_fpt), subtract_i64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_ipt), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_ipt, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_i64), divide_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_f64), divide_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_f64), divide_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_i64), divide_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_ipt, tyid_ipt), divide_ipt_ipt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_fpt), divide_fpt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_ipt, tyid_fpt), divide_ipt_fpt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_ipt), divide_fpt_ipt);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_ipt, tyid_i64), divide_ipt_i64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_f64), divide_fpt_f64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_ipt, tyid_f64), divide_ipt_f64);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_i64), divide_fpt_i64);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_ipt), divide_i64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_fpt), divide_f64_fpt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_ipt), divide_f64_ipt);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_fpt), divide_i64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_ipt), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_ipt, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_i64), point_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_f64), point_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_f64), point_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_i64), point_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_i64, tyid_i64), polar_i64_i64);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_f64, tyid_f64), polar_f64_f64);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_i64, tyid_f64), polar_i64_f64);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_f64, tyid_i64), polar_f64_i64);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_lst, tyid_i64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_i64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_f64, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_lst, tyid_lst), list_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_fpt, tyid_f64), rotate_fpt_fpt_f64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_fpt, tyid_f64), rotate_ipt_fpt_f64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_ipt, tyid_f64), rotate_fpt_ipt_f64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_fpt, tyid_i64), rotate_fpt_fpt_i64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_ipt, tyid_f64), rotate_ipt_ipt_f64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_fpt, tyid_i64), rotate_ipt_fpt_i64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_ipt, tyid_i64), rotate_fpt_ipt_i64);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_ipt, tyid_i64), rotate_ipt_ipt_i64);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_fpt, tyid_f64), list_lst_val_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_ipt, tyid_i64), list_lst_val_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_fpt, tyid_i64), list_lst_val_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_ipt, tyid_f64), list_lst_val_val);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_lst, tyid_f64), list_val_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_lst, tyid_i64), list_val_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_lst, tyid_i64), list_val_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_lst, tyid_f64), list_val_lst_val);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_fpt, tyid_lst), list_val_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_ipt, tyid_lst), list_val_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_ipt, tyid_lst), list_val_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_fpt, tyid_lst), list_val_val_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_lst, tyid_f64), list_lst_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_lst, tyid_i64), list_lst_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_fpt, tyid_lst), list_lst_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_ipt, tyid_lst), list_lst_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_lst, tyid_lst), list_val_lst_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_ipt, tyid_lst, tyid_lst), list_val_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_lst, tyid_lst), list_lst_lst_lst);

   register_n_arg(&mut indices, &mut fn_list, "list", list);

   register_n_arg(&mut indices, &mut fn_list, "each", each);
   register_n_arg(&mut indices, &mut fn_list, "each_with_last", each_with_last);
   register_n_arg(&mut indices, &mut fn_list, "each_with_index", each_with_index);

   (indices, fn_list)
}


#[macro_use]
mod macros;

mod number;
mod boolean;
mod functional;
mod point;
mod color;
mod text;
mod draw;
mod svg;

use std::any::TypeId;
use std::collections::HashMap;

use sys::ft::Face;
use draw::RGB;
use data::FloatPoint;

use super::compiler::{BuiltinIndices, FnRef};
use super::value_ptr::{ValuePtr, ValuePtrList};
use super::execute::Executor;
use super::parser::FnIndex;

use self::number::{
   add_f64_f64, multiply_f64_f64, subtract_f64_f64, divide_f64_f64,
   equal_f64_f64, unequal_f64_f64, sin_f64, cos_f64, pow_f64,
};

use self::boolean::{
   equal_bln_bln, unequal_bln_bln,
};

use self::point::{
   point_f64_f64,
   add_fpt_fpt, add_fpt_f64, add_f64_fpt,
   multiply_fpt_fpt, multiply_fpt_f64, multiply_f64_fpt,
   subtract_fpt_fpt, subtract_fpt_f64, subtract_f64_fpt,
   divide_fpt_fpt, divide_fpt_f64, divide_f64_fpt,
   polar_f64_f64, rotate_fpt_fpt_f64,
   equal_fpt_fpt, unequal_fpt_fpt,
   flip_x_fpt_f64_f64, flip_y_fpt_f64_f64,
};

use self::functional::{
   list, call_lst_fnp, call_lst_lst, each, each_with_last, each_with_index,
   zip, range, all, any, list_lst_val, list_val_lst, list_lst_lst,
   list_lst_val_val, list_val_lst_val, list_val_val_lst,
   list_lst_lst_val, list_lst_val_lst, list_val_lst_lst, list_lst_lst_lst,
   repeat,
};

use self::color::{
   rgb, equal_rgb_rgb, unequal_rgb_rgb,
};

use self::draw::{
   solid_fill,
};

use self::text::{
   font_face, text_fce_str_f64_fpt,
};

use self::svg::svg_path;


type CALL = fn(&[&ValuePtr], &Executor, &FnRef) -> Vec<ValuePtr>;

type HMA1R1 = HashMap<TypeId, CALL>;

type HMA2R1 = HashMap<(TypeId, TypeId), CALL>;

type HMA3R1 = HashMap<(TypeId, TypeId, TypeId), CALL>;

type HMA4R1 = HashMap<(TypeId, TypeId, TypeId, TypeId), CALL>;


pub enum TypeFnMap {
   HMA1R1(HMA1R1),
   HMA2R1(HMA2R1),
   HMA3R1(HMA3R1),
   HMA4R1(HMA4R1),
   CALL(CALL),
}


pub type FnList = Vec<TypeFnMap>;


pub struct BuiltinFns {
   pub fn_list: FnList,
   pub equal_ref: FnRef,
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


define_register_func!(register_1_arg, TypeId, HMA1R1);

define_register_func!(register_2_arg, (TypeId, TypeId), HMA2R1);

define_register_func!(register_3_arg, (TypeId, TypeId, TypeId), HMA3R1);

define_register_func!(register_4_arg, (TypeId, TypeId, TypeId, TypeId), HMA4R1);


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


pub fn register_builtin_fns() -> (BuiltinIndices, BuiltinFns) {
   let mut indices = BuiltinIndices::new();

   let mut fn_list = FnList::new();

   let tyid_i64 = TypeId::of::<i64>();
   let tyid_f64 = TypeId::of::<f64>();
   let tyid_lst = TypeId::of::<ValuePtrList>();
   let tyid_fpt = TypeId::of::<FloatPoint>();
   let tyid_fnp = TypeId::of::<FnRef>();
   let tyid_rgb = TypeId::of::<RGB>();
   let tyid_fce = TypeId::of::<Face>();
   let tyid_str = TypeId::of::<String>();
   let tyid_bln = TypeId::of::<bool>();

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_f64), add_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_fpt), add_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_f64), add_fpt_f64);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_fpt), add_f64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "add", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_f64), multiply_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_fpt), multiply_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_f64), multiply_fpt_f64);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_fpt), multiply_f64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "multiply", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_f64), subtract_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_fpt), subtract_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_f64), subtract_fpt_f64);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_fpt), subtract_f64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "subtract", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_f64), divide_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_fpt), divide_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_f64), divide_fpt_f64);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_fpt), divide_f64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "divide", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_bln, tyid_bln), equal_bln_bln);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_f64, tyid_f64), equal_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_fpt, tyid_fpt), equal_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "equal", (tyid_rgb, tyid_rgb), equal_rgb_rgb);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_bln, tyid_bln), unequal_bln_bln);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_f64, tyid_f64), unequal_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_fpt, tyid_fpt), unequal_fpt_fpt);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_lst, tyid_fpt), list_lst_val);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_f64, tyid_lst), list_val_lst);
   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_fpt, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "unequal", (tyid_rgb, tyid_rgb), unequal_rgb_rgb);

   register_2_arg(&mut indices, &mut fn_list, "pow", (tyid_f64, tyid_f64), pow_f64);

   register_1_arg(&mut indices, &mut fn_list, "sin", tyid_f64, sin_f64);

   register_1_arg(&mut indices, &mut fn_list, "cos", tyid_f64, cos_f64);

   register_1_arg(&mut indices, &mut fn_list, "all", tyid_lst, all);

   register_1_arg(&mut indices, &mut fn_list, "any", tyid_lst, any);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_f64), point_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_f64, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "point", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_f64, tyid_f64), polar_f64_f64);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_lst, tyid_f64), list_lst_val);
   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_f64, tyid_lst), list_val_lst);

   register_2_arg(&mut indices, &mut fn_list, "polar", (tyid_lst, tyid_lst), list_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_fpt, tyid_f64), rotate_fpt_fpt_f64);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_fpt, tyid_f64), list_lst_val_val);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_lst, tyid_f64), list_val_lst_val);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_fpt, tyid_lst), list_val_val_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_lst, tyid_f64), list_lst_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_fpt, tyid_lst), list_lst_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_fpt, tyid_lst, tyid_lst), list_val_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "rotate", (tyid_lst, tyid_lst, tyid_lst), list_lst_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "flip_x", (tyid_fpt, tyid_f64, tyid_f64), flip_x_fpt_f64_f64);

   register_3_arg(&mut indices, &mut fn_list, "flip_x", (tyid_lst, tyid_f64, tyid_f64), list_lst_val_val);

   register_3_arg(&mut indices, &mut fn_list, "flip_y", (tyid_fpt, tyid_f64, tyid_f64), flip_y_fpt_f64_f64);

   register_3_arg(&mut indices, &mut fn_list, "flip_y", (tyid_lst, tyid_f64, tyid_f64), list_lst_val_val);

   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_f64, tyid_f64, tyid_f64), rgb);

   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_lst, tyid_f64, tyid_f64), list_lst_val_val);
   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_f64, tyid_lst, tyid_f64), list_val_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_f64, tyid_f64, tyid_lst), list_val_val_lst);

   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_lst, tyid_lst, tyid_f64), list_lst_lst_val);
   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_lst, tyid_f64, tyid_lst), list_lst_val_lst);
   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_f64, tyid_lst, tyid_lst), list_val_lst_lst);

   register_3_arg(&mut indices, &mut fn_list, "rgb", (tyid_lst, tyid_lst, tyid_lst), list_lst_lst_lst);

   register_1_arg(&mut indices, &mut fn_list, "font_face", tyid_str, font_face);

   register_4_arg(&mut indices, &mut fn_list, "text", (tyid_fce, tyid_str, tyid_f64, tyid_fpt), text_fce_str_f64_fpt);

   register_2_arg(&mut indices, &mut fn_list, "fill", (tyid_lst, tyid_rgb), solid_fill);
   register_2_arg(&mut indices, &mut fn_list, "fill", (tyid_lst, tyid_lst), list_lst_lst);

   register_2_arg(&mut indices, &mut fn_list, "call", (tyid_lst, tyid_fnp), call_lst_fnp);
   register_2_arg(&mut indices, &mut fn_list, "call", (tyid_lst, tyid_lst), call_lst_lst);

   register_n_arg(&mut indices, &mut fn_list, "list", list);

   register_n_arg(&mut indices, &mut fn_list, "zip", zip);

   register_2_arg(&mut indices, &mut fn_list, "each", (tyid_lst, tyid_fnp), each);
   register_2_arg(&mut indices, &mut fn_list, "each_with_last", (tyid_lst, tyid_fnp), each_with_last);
   register_2_arg(&mut indices, &mut fn_list, "each_with_index", (tyid_lst, tyid_fnp), each_with_index);

   register_2_arg(&mut indices, &mut fn_list, "range", (tyid_i64, tyid_i64), range);

   register_1_arg(&mut indices, &mut fn_list, "svg_path", tyid_str, svg_path);

   register_2_arg(&mut indices, &mut fn_list, "repeat", (tyid_i64, tyid_fnp), repeat);

   let equal_ref = FnRef::builtin(indices.get("equal").unwrap().index);

   let fns = BuiltinFns {
      fn_list: fn_list,
      equal_ref: equal_ref,
   };

   (indices, fns)
}


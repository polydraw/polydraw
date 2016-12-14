use std::any::TypeId;

use draw::RGB;
use devel::Poly;
use data::{IntPoint, FloatPoint, Empty};

use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::compiler::FnRef;
use super::execute::Executor;



pub fn solid_fill(
   arguments: &[&ValuePtr],
   _: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let color = value_ptr_as_ref!(arguments[1], RGB);

   let mut points = Vec::new();

   if extract_poly_points(list, &mut points) {
      vecval!(
         Poly::new(points, color.clone())
      )
   } else {
      vecval!(Empty)
   }
}


fn extract_poly_points(list: &ValuePtrList, points: &mut Vec<Vec<IntPoint>>) -> bool {
   if list.len() == 0 {
      return true;
   }

   if TypeId::of::<ValuePtrList>() == list[0].type_id {
      for value_ptr in list.iter() {
         if TypeId::of::<ValuePtrList>() == value_ptr.type_id {
            let inner = value_ptr_as_ref!(value_ptr, ValuePtrList);

            if !extract_poly_points(inner, points) {
               return false;
            }
         } else {
            return false;
         }
      }
   } else {
      if list.len() < 3 {
         return false;
      }

      let mut inner_points: Vec<IntPoint> = Vec::new();

      for value_ptr in list.iter() {
         if TypeId::of::<IntPoint>() == value_ptr.type_id {
            let point = value_ptr_as_ref!(value_ptr, IntPoint);

            inner_points.push(point.clone());
         } else if TypeId::of::<IntPoint>() == value_ptr.type_id {
            let point = value_ptr_as_ref!(value_ptr, FloatPoint);

            inner_points.push(point.as_int());
         } else {
            return false;
         }
      }

      points.push(inner_points);
   }

   true
}


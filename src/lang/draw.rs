use std::any::TypeId;
use std::usize;

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

   if list.len() == 0 {
      return vecval!(Empty);
   }

   let color = value_ptr_as_ref!(arguments[1], RGB);

   let depth = drill_points_depth(list);

   match depth {
      Some(depth_value) => vec![extract_poly_points(list, color, depth_value)],
      None => vecval!(Empty),
   }
}


fn extract_poly_points(list: &ValuePtrList, color: &RGB, depth: usize) -> ValuePtr {
   match depth {
      1 => {
         let flat_points = extract_flat_points(list);

         if flat_points.len() < 3 {
            ValuePtr::new(Empty)
         } else {
            ValuePtr::new(
               Poly::new(vec![flat_points], color.clone())
            )
         }
      },
      2 => {
         let mut contours = Vec::new();

         for value_ptr in list.iter() {
            assert_eq!(TypeId::of::<ValuePtrList>(), value_ptr.type_id);

            let inner_list = value_ptr_as_ref!(value_ptr, ValuePtrList);

            let flat_points = extract_flat_points(inner_list);

            if flat_points.len() < 3 {
               continue;
            }

            contours.push(flat_points);
         }

         if contours.len() == 0 {
            ValuePtr::new(Empty)
         } else {
            ValuePtr::new(
               Poly::new(contours, color.clone())
            )
         }
      },
      _ => {
         let mut ptr_lists = Vec::new();

         for value_ptr in list.iter() {
            assert_eq!(TypeId::of::<ValuePtrList>(), value_ptr.type_id);

            let inner_list = value_ptr_as_ref!(value_ptr, ValuePtrList);

            let ptr_list = extract_poly_points(inner_list, color, depth - 1);

            ptr_lists.push(ptr_list);
         }

         ValuePtr::new(ptr_lists)
      },
   }
}


fn extract_flat_points(list: &ValuePtrList) -> Vec<IntPoint> {
   let tyid_ipt = TypeId::of::<IntPoint>();
   let tyid_fpt = TypeId::of::<FloatPoint>();

   let mut points = Vec::new();

   for value_ptr in list.iter() {
      if tyid_ipt == value_ptr.type_id {
         let point = value_ptr_as_ref!(value_ptr, IntPoint);

         points.push(point.clone());
      } else if tyid_fpt == value_ptr.type_id {
         let point = value_ptr_as_ref!(value_ptr, FloatPoint);

         points.push(point.as_int());
      }
   }

   points
}


fn drill_points_depth(list: &ValuePtrList) -> Option<usize> {
   let mut depth: Option<usize> = None;

   let tyid_lst = TypeId::of::<ValuePtrList>();
   let tyid_ipt = TypeId::of::<IntPoint>();
   let tyid_fpt = TypeId::of::<FloatPoint>();

   for value_ptr in list.iter() {
      if tyid_lst == value_ptr.type_id {
         let inner_list = value_ptr_as_ref!(value_ptr, ValuePtrList);

         let inner_depth = drill_points_depth(inner_list);

         if let Some(inner_depth_value) = inner_depth {
            if let Some(depth_value) = depth {
               if depth_value != inner_depth_value + 1 {
                  return None;
               }
            } else {
               depth = Some(inner_depth_value + 1)
            }
         } else {
            return None;
         }
      } else if tyid_ipt != value_ptr.type_id && tyid_fpt != value_ptr.type_id {
         return None;
      }
   }

   match depth {
      Some(_) => depth,
      None => Some(1),
   }
}


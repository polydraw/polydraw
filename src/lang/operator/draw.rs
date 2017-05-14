use std::any::TypeId;
use std::usize;

use draw::RGB;
use devel::Poly;
use data::{IntPoint, FloatPoint, Empty};

use lang::variant::{Variant, VariantVec};
use lang::compiler::FnRef;
use lang::execute::Executor;



pub fn solid_fill(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let list = arguments[0].as_ref::<VariantVec>();

   if list.len() == 0 {
      return vecval!(executor, Empty);
   }

   let color = arguments[1].as_ref::<RGB>();

   let depth = drill_points_depth(list);

   match depth {
      Some(depth_value) => vec![extract_poly_points(executor, list, color, depth_value)],
      None => vecval!(executor, Empty),
   }
}


fn extract_poly_points(executor: &Executor, list: &VariantVec, color: &RGB, depth: usize) -> Variant {
   match depth {
      1 => {
         let flat_points = extract_flat_points(list);

         if flat_points.len() < 3 {
            executor.registry.variant(Empty)
         } else {
            executor.registry.variant(
               Poly::new(vec![flat_points], color.clone())
            )
         }
      },
      2 => {
         let mut contours = Vec::new();

         for variant in list.iter() {
            assert_eq!(TypeId::of::<VariantVec>(), *variant.type_id());

            let inner_list = variant.as_ref::<VariantVec>();

            let flat_points = extract_flat_points(inner_list);

            if flat_points.len() < 3 {
               continue;
            }

            contours.push(flat_points);
         }

         if contours.len() == 0 {
            executor.registry.variant(Empty)
         } else {
            executor.registry.variant(
               Poly::new(contours, color.clone())
            )
         }
      },
      _ => {
         let mut ptr_lists = Vec::new();

         for variant in list.iter() {
            assert_eq!(TypeId::of::<VariantVec>(), *variant.type_id());

            let inner_list = variant.as_ref::<VariantVec>();

            let ptr_list = extract_poly_points(executor, inner_list, color, depth - 1);

            ptr_lists.push(ptr_list);
         }

         executor.registry.variant(ptr_lists)
      },
   }
}


fn extract_flat_points(list: &VariantVec) -> Vec<IntPoint> {
   let mut points = Vec::new();

   for variant in list.iter() {
      if let Some(point) = variant.as_ref_checked::<IntPoint>() {
         points.push(point.clone());
      } else if let Some(point) = variant.as_ref_checked::<FloatPoint>() {
         points.push(point.as_int());
      }
   }

   points
}


fn drill_points_depth(list: &VariantVec) -> Option<usize> {
   let mut depth: Option<usize> = None;

   let tyid_ipt = TypeId::of::<IntPoint>();
   let tyid_fpt = TypeId::of::<FloatPoint>();

   for variant in list.iter() {
      if let Some(inner_list) = variant.as_ref_checked::<VariantVec>() {
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
      } else if tyid_ipt != *variant.type_id() && tyid_fpt != *variant.type_id() {
         return None;
      }
   }

   match depth {
      Some(_) => depth,
      None => Some(1),
   }
}


use std::collections::HashMap;
use std::any::TypeId;
use std::fmt;

use super::variant::{Variant, VTable, VType, CloneFn, DropFn, DebugFn};
use super::compiler::FnRef;

use sys::ft::Face;
use draw::RGB;
use devel::Poly;
use data::{IntPoint, FloatPoint, Empty};


pub struct TypeRegistry {
   vtypes: HashMap<TypeId, VType>,
}


impl TypeRegistry {
   pub fn new() -> Self {
      let mut registry = TypeRegistry {
         vtypes: HashMap::new(),
      };

      registry.register::<i64>(clone_i64, drop_i64, debug_i64);
      registry.register::<f64>(clone_f64, drop_f64, debug_f64);
      registry.register::<bool>(clone_bool, drop_bool, debug_bool);
      registry.register::<String>(clone_string, drop_string, debug_string);
      registry.register::<Empty>(clone_empty, drop_empty, debug_empty);
      registry.register::<FnRef>(clone_fnref, drop_fnref, debug_fnref);
      registry.register::<IntPoint>(clone_int_point, drop_int_point, debug_int_point);
      registry.register::<FloatPoint>(clone_float_point, drop_float_point, debug_float_point);
      registry.register::<RGB>(clone_rgb, drop_rgb, debug_rgb);
      registry.register::<Poly>(clone_poly, drop_poly, debug_poly);
      registry.register::<Face>(clone_face, drop_face, debug_face);

      registry
   }

   fn register<T: 'static>(
      &mut self,
      clone: CloneFn,
      drop: DropFn,
      debug: DebugFn,
   ) {
      let type_id = TypeId::of::<T>();

      self.vtypes.insert(
         type_id,
         VType {
            type_id: type_id,
            vtable: VTable {
               clone: clone,
               drop: drop,
               debug: debug,
            }
         }
      );
   }

   pub fn variant<T: 'static>(&self, value: T) -> Variant {
      let vtype = self.vtype::<T>();

      Variant::new(value, vtype)
   }

   fn vtype<'a, T: 'static>(&'a self) -> &VType {
      let type_id = TypeId::of::<T>();

      &self.vtypes[&type_id]
   }
}


macro_rules! drop_func {
   ($drop_name:ident, $ty:ty) => {
      fn $drop_name(value: &mut Variant) {
         drop(
            unsafe { *Box::from_raw(value.data as *mut $ty) }
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
drop_func!(drop_rgb, RGB);
drop_func!(drop_poly, Poly);
drop_func!(drop_face, Face);


macro_rules! clone_func {
   ($clone_name:ident, $ty:ty) => {
      fn $clone_name(value: &Variant) -> Variant {
         unsafe {
            Variant::new(value.as_ref::<$ty>().clone(), &*value.vtype as &VType)
         }
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


macro_rules! debug_func {
   ($debug_name:ident, $ty:ty) => {
      fn $debug_name(value: &Variant, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{:?}", value.as_ref::<$ty>())
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
debug_func!(debug_rgb, RGB);
debug_func!(debug_poly, Poly);
debug_func!(debug_face, Face);

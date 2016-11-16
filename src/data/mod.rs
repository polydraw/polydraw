pub mod point;

pub use self::point::{IntPoint, FloatPoint, min_max_by_x};


pub fn min_max<T: PartialOrd>(first: T, second: T) -> (T, T) {
   if first < second {
      (first, second)
   } else {
      (second, first)
   }
}

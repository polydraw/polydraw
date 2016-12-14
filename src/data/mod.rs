pub mod point;
pub mod empty;

pub use self::point::{IntPoint, FloatPoint, min_max_by_x};
pub use self::empty::Empty;


pub fn min_max<T: PartialOrd>(first: T, second: T) -> (T, T) {
   if first < second {
      (first, second)
   } else {
      (second, first)
   }
}

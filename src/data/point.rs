use std::fmt;
use std::cmp::{PartialOrd, Ordering};
use std::ops::{Mul, MulAssign};


macro_rules! point_struct {
   ($name:ident, $point_ty:ty, $( $derive:ident ),*) => {

      #[derive($( $derive ),*)]
      pub struct $name {
         pub x: $point_ty,
         pub y: $point_ty
      }

      impl $name {
         #[inline]
         pub fn new(x: $point_ty, y: $point_ty) -> Self {
            $name {
               x: x,
               y: y
            }
         }

         #[inline]
         pub fn update(&mut self, x: $point_ty, y: $point_ty) {
            self.x = x;
            self.y = y;
         }
      }

      impl MulAssign for $name {
          fn mul_assign(&mut self, _rhs: $name) {
            self.x *= _rhs.x;
            self.y *= _rhs.y;
          }
      }

      impl Mul<$point_ty> for $name {
         type Output = $name;

         #[inline]
         fn mul(self, val: $point_ty) -> $name {
            $name {
               x: self.x * val,
               y: self.y * val,
            }
         }
      }

      impl fmt::Debug for $name {
         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<{} {}>", self.x, self.y)
         }
      }
   }
}

point_struct!(IntPoint, i64, PartialEq, Eq, Clone, Copy, Default);
point_struct!(FloatPoint, f64, Clone, Copy, Default);


impl PartialOrd for IntPoint {
   fn partial_cmp(&self, other: &IntPoint) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for IntPoint {
   fn cmp(&self, other: &IntPoint) -> Ordering {
      if self.y < other.y {
         Ordering::Less
      } else if self.y > other.y {
         Ordering::Greater
      } else if self.x < other.x {
         Ordering::Less
      } else if self.x > other.x {
         Ordering::Greater
      } else {
         Ordering::Equal
      }
   }
}


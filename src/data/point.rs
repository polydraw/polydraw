use std::fmt;
use std::cmp::{PartialOrd, Ordering};
use std::ops::{Mul, MulAssign, Add, Sub, Div};


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

      impl Add<$name> for $name {
         type Output = $name;

         #[inline]
         fn add(self, val: $name) -> $name {
            $name {
               x: self.x + val.x,
               y: self.y + val.y,
            }
         }
      }

      impl Sub<$name> for $name {
         type Output = $name;

         #[inline]
         fn sub(self, val: $name) -> $name {
            $name {
               x: self.x - val.x,
               y: self.y - val.y,
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


impl Mul<i64> for IntPoint {
   type Output = IntPoint;

   #[inline]
   fn mul(self, val: i64) -> IntPoint {
      IntPoint {
         x: self.x * val,
         y: self.y * val,
      }
   }
}


impl Div<i64> for IntPoint {
   type Output = IntPoint;

   #[inline]
   fn div(self, val: i64) -> IntPoint {
      IntPoint {
         x: self.x / val,
         y: self.y / val,
      }
   }
}


impl Mul<f64> for FloatPoint {
   type Output = FloatPoint;

   #[inline]
   fn mul(self, val: f64) -> FloatPoint {
      FloatPoint {
         x: self.x * val,
         y: self.y * val,
      }
   }
}


impl Div<f64> for FloatPoint {
   type Output = FloatPoint;

   #[inline]
   fn div(self, val: f64) -> FloatPoint {
      FloatPoint {
         x: self.x / val,
         y: self.y / val,
      }
   }
}


impl Mul<i64> for FloatPoint {
   type Output = FloatPoint;

   #[inline]
   fn mul(self, val: i64) -> FloatPoint {
      FloatPoint {
         x: self.x * val as f64,
         y: self.y * val as f64,
      }
   }
}


impl Div<i64> for FloatPoint {
   type Output = FloatPoint;

   #[inline]
   fn div(self, val: i64) -> FloatPoint {
      FloatPoint {
         x: self.x / val as f64,
         y: self.y / val as f64,
      }
   }
}


impl IntPoint {
   pub fn as_float(&self) -> FloatPoint {
      FloatPoint::new(self.x as f64, self.y as f64)
   }
}


impl FloatPoint {
   pub fn as_int(&self) -> IntPoint {
      IntPoint::new(self.x.round() as i64, self.y.round() as i64)
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


pub fn min_max_by_x(a: IntPoint, b: IntPoint) -> (IntPoint, IntPoint) {
   if a.x < b.x {
      (a, b)
   } else if a.x > b.x {
      (b, a)
   } else if a.y < b.y {
      (a, b)
   } else if a.y > b.y {
      (b, a)
   } else {
      (a, b)
   }
}


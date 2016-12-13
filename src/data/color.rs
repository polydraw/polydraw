use std::fmt;


#[derive(PartialEq, Eq, Clone)]
pub struct Rgb {
   pub r: i64,
   pub g: i64,
   pub b: i64
}

impl Rgb {
   pub fn new(r: i64, g: i64, b: i64) -> Self {
      Rgb {
         r: r,
         g: g,
         b: b
      }
   }
}

impl fmt::Debug for Rgb {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "(rgb! {} {} {})", self.r, self.g, self.b)
   }
}


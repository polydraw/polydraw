use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct RGB {
   pub r: u8,
   pub g: u8,
   pub b: u8
}

impl RGB {
   pub fn new(r: u8, g: u8, b: u8) -> Self {
      RGB {
         r: r,
         g: g,
         b: b
      }
   }
}

impl Default for RGB {
   fn default() -> RGB {
      RGB::new(0, 0, 0)
   }
}

impl fmt::Debug for RGB {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "(rgb! {} {} {})", self.r, self.g, self.b)
   }
}

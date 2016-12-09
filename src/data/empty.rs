use std::fmt;


#[derive(Clone)]
pub struct Empty;

impl fmt::Debug for Empty {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "empty")
   }
}

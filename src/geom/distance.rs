use super::number::Number;

pub trait Distance<Q, T> where T: Number {
   fn distance(&self, other: &Self) -> T;
}

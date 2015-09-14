use super::coordinate::Coordinate;

pub trait Distance<Q, T> where T: Coordinate {
   fn distance(&self, other: &Self) -> T;
}

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::num::Zero;

use super::number::NumberOps;

pub trait Coordinate:
   Add<Self, Output=Self> +
   Sub<Self, Output=Self> +
   Mul<Self, Output=Self> +
   Div<Self, Output=Self> +
   Neg<Output=Self> +
   PartialEq +
   NumberOps +
   Zero +
   Default +
   Copy +
   Clone {}

impl Coordinate for f64 {}

impl Coordinate for f32 {}

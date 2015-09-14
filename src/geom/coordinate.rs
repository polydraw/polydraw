use std::ops::{Add, Sub, Mul};

use super::float::FloatOps;

pub trait Coordinate:
   Add<Self, Output=Self> +
   Sub<Self, Output=Self> +
   Mul<Self, Output=Self> +
   FloatOps +
   Default +
   Copy +
   Clone {}

impl Coordinate for f64 {}

impl Coordinate for f32 {}

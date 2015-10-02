use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::{PartialEq, PartialOrd};
use std::num::{Zero, One};
use std::intrinsics;

pub trait Number:
   Add<Self, Output=Self> +
   Sub<Self, Output=Self> +
   Mul<Self, Output=Self> +
   Div<Self, Output=Self> +
   Neg<Output=Self> +
   PartialEq +
   PartialOrd +
   Zero +
   One +
   Default +
   Copy +
   Clone {}

impl Number for i64 {}

impl Number for i32 {}

use std::fmt;

pub use devel::Poly;
pub use draw::RGB;
pub use data::IntPoint;


#[derive(Debug, Clone)]
pub struct Layer {
   pub polys: Vec<Poly>,
}

impl Layer {
   #[inline]
   pub fn new(polys: Vec<Poly>) -> Self {
      Layer {
         polys: polys,
      }
   }
}

#[derive(Clone)]
pub struct Rect {
   pub p1: IntPoint,
   pub p2: IntPoint,
}

impl Rect {
   #[inline]
   pub fn new(p1: IntPoint, p2: IntPoint) -> Self {
      Rect {
         p1: p1,
         p2: p2,
      }
   }
}

impl fmt::Debug for Rect {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "(rect! {:?} {:?})", self.p1, self.p2)
   }
}


#[derive(Clone)]
pub struct Range<T> {
   pub start: T,
   pub end: T,
}

impl<T> Range<T> {
   #[inline]
   pub fn new(start: T, end: T) -> Self {
      Range {
         start: start,
         end: end,
      }
   }
}

impl<T> fmt::Debug for Range<T> where T: fmt::Display {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} .. {}", self.start, self.end)
   }
}


pub type PointList = Vec<IntPoint>;
pub type PointListList = Vec<Vec<IntPoint>>;
pub type RgbList = Vec<RGB>;
pub type PolyList = Vec<Poly>;
pub type LayerList = Vec<Layer>;
pub type IntRange = Range<i64>;
pub type FloatRange = Range<f64>;


#[derive(Clone)]
pub enum Data {
   None,
   Int(i64),
   Float(f64),
   Bool(bool),
   IntRange(IntRange),
   FloatRange(FloatRange),
   Point(IntPoint),
   Rgb(RGB),
   FunctionRef(String),
   Rect(Box<Rect>),
   Poly(Box<Poly>),
   Layer(Box<Layer>),
   IntList(Box<Vec<i64>>),
   FloatList(Box<Vec<f64>>),
   BoolList(Box<Vec<bool>>),
   PointList(Box<PointList>),
   PointListList(Box<PointListList>),
   RgbList(Box<Vec<RGB>>),
   PolyList(Box<PolyList>),
   LayerList(Box<LayerList>),
   DataList(Box<Vec<Data>>),
}

macro_rules! write_value {
   ($f:ident, $value:ident) => {
      write!($f, "{:?}", $value)
   }
}

macro_rules! write_list {
   ($f:ident, $list:ident) => {
      {
         if $list.len() == 0 {
            return write!($f, "[]");
         }

         write!($f, "[{:?}", $list[0]).unwrap();

         for element in &$list[1..] {
            write!($f, " {:?}", element).unwrap();
         }

         write!($f, "]")
      }
   }
}

impl fmt::Debug for Data {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &Data::None => write!(f, "None"),
         &Data::Int(ref value) => write_value!(f, value),
         &Data::Float(ref value) => write_value!(f, value),
         &Data::Bool(ref value) => write_value!(f, value),
         &Data::IntRange(ref value) => write_value!(f, value),
         &Data::FloatRange(ref value) => write_value!(f, value),
         &Data::Point(ref value) => write_value!(f, value),
         &Data::Rgb(ref value) => write_value!(f, value),
         &Data::Rect(ref value) => write_value!(f, value),
         &Data::FunctionRef(ref value) => write!(f, "@{}", value),
         &Data::Poly(ref value) => write!(f, "(poly! {:?})", value),
         &Data::Layer(ref value) => write!(f, "(layer! {:?})", value),
         &Data::IntList(ref list) => write_list!(f, list),
         &Data::FloatList(ref list) => write_list!(f, list),
         &Data::BoolList(ref list) => write_list!(f, list),
         &Data::PointList(ref list) => write_list!(f, list),
         &Data::PointListList(ref list) => write_list!(f, list),
         &Data::RgbList(ref list) => write_list!(f, list),
         &Data::PolyList(ref list) => write_list!(f, list),
         &Data::LayerList(ref list) => write_list!(f, list),
         &Data::DataList(ref list) => write_list!(f, list),
      }
   }
}

pub const NONE: Data = Data::None;

pub use devel::Poly;
pub use draw::RGB;
pub use geom::point::Point;


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

#[derive(Debug, Clone)]
pub struct BBox {
   pub p1: Point,
   pub p2: Point,
}

impl BBox {
   #[inline]
   pub fn new(p1: Point, p2: Point) -> Self {
      BBox {
         p1: p1,
         p2: p2,
      }
   }
}

pub type PointList = Vec<Point>;
pub type PointListList = Vec<Vec<Point>>;
pub type RgbList = Vec<RGB>;
pub type PolyList = Vec<Poly>;
pub type LayerList = Vec<Layer>;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataType {
   None,
   Int,
   Float,
   Bool,
   Point,
   Rgb,
   BBox,
   Poly,
   IntList,
   FloatList,
   BoolList,
   PointList,
   PointListList,
   PolyList,
   RgbList,
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Data {
   None,
   Int(i64),
   Float(f64),
   Bool(bool),
   Point(Point),
   Rgb(RGB),
   BBox(Box<BBox>),
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
}

pub const NONE: Data = Data::None;

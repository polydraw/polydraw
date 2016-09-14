use devel::Poly;

#[derive(Debug, Clone)]
pub struct Layer {
   pub polys: Vec<Box<Poly>>,
}

impl Layer {
   #[inline]
   pub fn new(polys: Vec<Box<Poly>>) -> Self {
      Layer {
         polys: polys,
      }
   }
}


pub type T3U8 = (u8, u8, u8);

pub type T2I64 = (i64, i64);
pub type VT2I64 = Vec<T2I64>;
pub type VVT2I64 = Vec<Vec<T2I64>>;

pub type T2T2I64 = ((i64, i64), (i64, i64));
pub type T2T2I64Box = Box<T2T2I64>;

pub type PolyBox = Box<Poly>;
pub type VPolyBox = Vec<Box<Poly>>;

pub type LayerBox = Box<Layer>;
pub type VLayerBox = Vec<Box<Layer>>;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Data {
   None,
   U8(u8),
   I64(i64),
   F64(f64),

   T2I64(T2I64),
   T2T2I64(T2T2I64Box),
   VT2I64(VT2I64),
   VVT2I64(VVT2I64),

   T3U8(T3U8),

   Poly(PolyBox),
   VPoly(VPolyBox),

   Layer(LayerBox),
   VLayer(VLayerBox),
}

pub const NONE: Data = Data::None;

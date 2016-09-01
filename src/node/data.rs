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


pub type U8U8U8 = (u8, u8, u8);

pub type I64I64 = (i64, i64);
pub type VI64I64 = Vec<I64I64>;
pub type VVI64I64 = Vec<Vec<I64I64>>;

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

   I64I64(I64I64),
   VI64I64(VI64I64),
   VVI64I64(VVI64I64),

   U8U8U8(U8U8U8),

   Poly(PolyBox),
   VPoly(VPolyBox),

   Layer(LayerBox),
   VLayer(VLayerBox),
}

pub const NONE: Data = Data::None;

use std::fmt;

use devel::Poly;
use geom::point::Point;
use draw::RGB;

use super::node::{Node, NodeRole};
use super::data::{Data, NONE, Layer, I64I64, VI64I64, VVI64I64, U8U8U8};


pub trait Operator where Self: fmt::Debug {
   fn new() -> Self where Self: Sized;

   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data;

   fn role(&self) -> NodeRole {
      NodeRole::Processor
   }
}


#[derive(Debug)]
pub struct NoneOperator { }

impl Operator for NoneOperator {
   #[inline]
   fn new() -> Self {
      NoneOperator { }
   }

   #[inline]
   fn process(&self, _: &Node, _: &mut [Vec<Data>]) -> Data {
      NONE
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct DataOperator { }

impl Operator for DataOperator {
   #[inline]
   fn new() -> Self {
      DataOperator { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      node.input(state, 0)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct Add { }

impl Operator for Add {
   #[inline]
   fn new() -> Self {
      Add { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      match (in1, in2) {
         (Data::I64(v1), Data::I64(v2)) => <(i64, i64)>::add(v1, v2),

         (Data::F64(v1), Data::I64(v2)) => <(f64, i64)>::add(v1, v2),
         (Data::I64(v1), Data::F64(v2)) => <(f64, i64)>::add(v2, v1),

         (Data::I64I64(v1), Data::I64(v2)) => <(I64I64, i64)>::add(v1, v2),
         (Data::I64(v1), Data::I64I64(v2)) => <(I64I64, i64)>::add(v2, v1),

         (Data::VI64I64(v1), Data::I64I64(v2)) => <(VI64I64, I64I64)>::add(v1, v2),
         (Data::I64I64(v1), Data::VI64I64(v2)) => <(VI64I64, I64I64)>::add(v2, v1),

         (Data::VVI64I64(v1), Data::I64I64(v2)) => <(VVI64I64, I64I64)>::add(v1, v2),
         (Data::I64I64(v1), Data::VVI64I64(v2)) => <(VVI64I64, I64I64)>::add(v2, v1),

         _ => NONE
      }
   }
}

trait AddTrait<T1, T2> {
   fn add(v1: T1, v2: T2) -> Data;
}

impl AddTrait<i64, i64> for (i64, i64) {
   #[inline]
   fn add(v1: i64, v2: i64) -> Data {
      Data::I64(v1 + v2)
   }
}

impl AddTrait<f64, i64> for (f64, i64) {
   #[inline]
   fn add(v1: f64, v2: i64) -> Data {
      Data::F64(v1 + v2 as f64)
   }
}

impl AddTrait<I64I64, i64> for (I64I64, i64) {
   #[inline]
   fn add(mut v1: I64I64, v2: i64) -> Data {
      v1.0 += v2;
      v1.1 += v2;

      Data::I64I64(v1)
   }
}

impl AddTrait<VI64I64, I64I64> for (VI64I64, I64I64) {
   #[inline]
   fn add(mut v1: VI64I64, v2: I64I64) -> Data {
      for tuple in &mut v1 {
         tuple.0 += v2.0;
         tuple.1 += v2.1;
      }

      Data::VI64I64(v1)
   }
}

impl AddTrait<VVI64I64, I64I64> for (VVI64I64, I64I64) {
   #[inline]
   fn add(mut v1: VVI64I64, v2: I64I64) -> Data {
      for src in &mut v1 {
         for tuple in src.iter_mut() {
            tuple.0 += v2.0;
            tuple.1 += v2.1;
         }
      }

      Data::VVI64I64(v1)
   }
}


#[derive(Debug)]
pub struct Join { }

impl Join {
   #[inline]
   fn process_two(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      match (in1, in2) {
         (Data::I64(v1), Data::I64(v2)) => Data::I64I64((v1, v2)),
         _ => NONE
      }
   }

   #[inline]
   fn process_three(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);
      let in3 = node.input(state, 2);

      match (in1, in2, in3) {
         (Data::U8(v1), Data::U8(v2), Data::U8(v3)) => Data::U8U8U8((v1, v2, v3)),
         _ => NONE
      }
   }
}

impl Operator for Join {
   #[inline]
   fn new() -> Self {
      Join { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      match node.len() {
         2 => self.process_two(node, state),
         3 => self.process_three(node, state),
         _ => Data::None
      }
   }
}


#[derive(Debug)]
pub struct BuildList { }

impl BuildList {
   #[inline]
   fn create_poly_list(&self, node: &Node, state: &mut [Vec<Data>], poly: Box<Poly>) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(poly);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Poly(poly) = input {
            result.push(poly);
         }
      }

      Data::VPoly(result)
   }

   #[inline]
   fn create_layer_list(&self, node: &Node, state: &mut [Vec<Data>], layer: Box<Layer>) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(layer);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Layer(layer) = input {
            result.push(layer);
         }
      }

      Data::VLayer(result)
   }
}

impl Operator for BuildList {
   #[inline]
   fn new() -> Self {
      BuildList { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let first = node.input(state, 0);

      match first {
         Data::Poly(poly) => self.create_poly_list(node, state, poly),
         Data::Layer(layer) => self.create_layer_list(node, state, layer),
         _ => NONE
      }
   }
}


#[derive(Debug)]
pub struct BuildPoly { }

impl Operator for BuildPoly {
   #[inline]
   fn new() -> Self {
      BuildPoly { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let points = node.input(state, 0);
      let color = node.input(state, 1);

      match (points, color) {
         (Data::VI64I64(v1), Data::U8U8U8(v2)) => <(VI64I64, U8U8U8)>::build_poly(v1, v2),
         _ => NONE
      }
   }
}

trait BuildPolyTrait<T1, T2> {
   fn build_poly(v1: T1, v2: T2) -> Data;
}

impl BuildPolyTrait<VI64I64, U8U8U8> for (VI64I64, U8U8U8) {
   #[inline]
   fn build_poly(array: VI64I64, color: U8U8U8) -> Data {
      let mut points = Vec::with_capacity(array.len());

      for tuple in array {
         points.push(Point::new(tuple.0, tuple.1))
      }

      let color = RGB::new(color.0, color.1, color.2);

      let poly = Poly::new(points, color);

      Data::Poly(Box::new(poly))
   }
}


#[derive(Debug)]
pub struct BuildLayer { }

impl Operator for BuildLayer {
   #[inline]
   fn new() -> Self {
      BuildLayer { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let polys_data = node.input(state, 0);

      match polys_data {
         Data::VPoly(polys) => {
            Data::Layer(
               Box::new(
                  Layer::new(polys)
               )
            )
         },
         _ => NONE
      }
   }
}


#[derive(Debug)]
pub struct BuildArtboard {
   list_node: BuildList,
}

impl Operator for BuildArtboard {
   #[inline]
   fn new() -> Self {
      BuildArtboard {
         list_node: BuildList::new()
      }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      self.list_node.process(node, state)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Artboard
   }
}


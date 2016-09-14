use std::fmt;
use std::i64;

use devel::Poly;
use geom::point::Point;
use draw::RGB;

use super::node::{Node, NodeRole};
use super::data::{Data, NONE, Layer, T2I64, VT2I64, VVT2I64, T3U8, T2T2I64};


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

         (Data::T2I64(v1), Data::I64(v2)) => <(T2I64, i64)>::add(v1, v2),
         (Data::I64(v1), Data::T2I64(v2)) => <(T2I64, i64)>::add(v2, v1),

         (Data::VT2I64(v1), Data::T2I64(v2)) => <(VT2I64, T2I64)>::add(v1, v2),
         (Data::T2I64(v1), Data::VT2I64(v2)) => <(VT2I64, T2I64)>::add(v2, v1),

         (Data::VVT2I64(v1), Data::T2I64(v2)) => <(VVT2I64, T2I64)>::add(v1, v2),
         (Data::T2I64(v1), Data::VVT2I64(v2)) => <(VVT2I64, T2I64)>::add(v2, v1),

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

impl AddTrait<T2I64, i64> for (T2I64, i64) {
   #[inline]
   fn add(mut v1: T2I64, v2: i64) -> Data {
      v1.0 += v2;
      v1.1 += v2;

      Data::T2I64(v1)
   }
}

impl AddTrait<VT2I64, T2I64> for (VT2I64, T2I64) {
   #[inline]
   fn add(mut v1: VT2I64, v2: T2I64) -> Data {
      for tuple in &mut v1 {
         tuple.0 += v2.0;
         tuple.1 += v2.1;
      }

      Data::VT2I64(v1)
   }
}

impl AddTrait<VVT2I64, T2I64> for (VVT2I64, T2I64) {
   #[inline]
   fn add(mut v1: VVT2I64, v2: T2I64) -> Data {
      for src in &mut v1 {
         for tuple in src.iter_mut() {
            tuple.0 += v2.0;
            tuple.1 += v2.1;
         }
      }

      Data::VVT2I64(v1)
   }
}


#[derive(Debug)]
pub struct Rotate { }

impl Operator for Rotate {
   #[inline]
   fn new() -> Self {
      Rotate { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in_target = node.input(state, 0);
      let in_origin = node.input(state, 1);
      let in_angle = node.input(state, 2);

      match (in_target, in_origin, in_angle) {
         (Data::T2I64(target), Data::T2I64(origin), Data::F64(angle)) =>
            <(T2I64, f64)>::rotate(target, origin, angle),

         (Data::T2I64(target), Data::T2I64(origin), Data::I64(angle)) =>
            <(T2I64, i64)>::rotate(target, origin, angle),

         (Data::VT2I64(target), Data::T2I64(origin), Data::F64(angle)) =>
            <(VT2I64, f64)>::rotate(target, origin, angle),

         (Data::VT2I64(target), Data::T2I64(origin), Data::I64(angle)) =>
            <(VT2I64, i64)>::rotate(target, origin, angle),

         (Data::VVT2I64(target), Data::T2I64(origin), Data::F64(angle)) =>
            <(VVT2I64, f64)>::rotate(target, origin, angle),

         (Data::VVT2I64(target), Data::T2I64(origin), Data::I64(angle)) =>
            <(VVT2I64, i64)>::rotate(target, origin, angle),

         _ => NONE
      }
   }
}

trait RotateTrait<T1, T2> {
   fn rotate(target: T1, origin: T2I64, angle: T2) -> Data;
}

impl RotateTrait<T2I64, f64> for (T2I64, f64) {
   #[inline]
   fn rotate(mut target: T2I64, origin: T2I64, angle: f64) -> Data {
      let cx = origin.0 as f64;
      let cy = origin.1 as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      let mut x = target.0 as f64;
      let mut y = target.1 as f64;

      x -= cx;
      y -= cy;

      target.0 = (x * c - y * s + cx) as i64;
      target.1 = (x * s + y * c + cy) as i64;

      Data::T2I64(target)
   }
}

impl RotateTrait<T2I64, i64> for (T2I64, i64) {
   #[inline]
   fn rotate(target: T2I64, origin: T2I64, angle: i64) -> Data {
      <(T2I64, f64)>::rotate(target, origin, angle as f64)
   }
}

impl RotateTrait<VT2I64, f64> for (VT2I64, f64) {
   #[inline]
   fn rotate(mut target: VT2I64, origin: T2I64, angle: f64) -> Data {
      let cx = origin.0 as f64;
      let cy = origin.1 as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for tuple in target.iter_mut() {
         let mut x = tuple.0 as f64;
         let mut y = tuple.1 as f64;

         x -= cx;
         y -= cy;

         tuple.0 = (x * c - y * s + cx) as i64;
         tuple.1 = (x * s + y * c + cy) as i64;
      }

      Data::VT2I64(target)
   }
}

impl RotateTrait<VT2I64, i64> for (VT2I64, i64) {
   #[inline]
   fn rotate(target: VT2I64, origin: T2I64, angle: i64) -> Data {
      <(VT2I64, f64)>::rotate(target, origin, angle as f64)
   }
}

impl RotateTrait<VVT2I64, f64> for (VVT2I64, f64) {
   #[inline]
   fn rotate(mut target: VVT2I64, origin: T2I64, angle: f64) -> Data {
      let cx = origin.0 as f64;
      let cy = origin.1 as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for outer in &mut target {
         for tuple in outer.iter_mut() {
            let mut x = tuple.0 as f64;
            let mut y = tuple.1 as f64;

            x -= cx;
            y -= cy;

            tuple.0 = (x * c - y * s + cx) as i64;
            tuple.1 = (x * s + y * c + cy) as i64;
         }
      }

      Data::VVT2I64(target)
   }
}

impl RotateTrait<VVT2I64, i64> for (VVT2I64, i64) {
   #[inline]
   fn rotate(target: VVT2I64, origin: T2I64, angle: i64) -> Data {
      <(VVT2I64, f64)>::rotate(target, origin, angle as f64)
   }
}

#[derive(Debug)]
pub struct BBox { }

impl Operator for BBox {
   #[inline]
   fn new() -> Self {
      BBox { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in_object = node.input(state, 0);

      match in_object {
         Data::T2I64(object) => <T2I64>::bbox(object),

         Data::VT2I64(object) => <VT2I64>::bbox(object),

         Data::VVT2I64(object) => <VVT2I64>::bbox(object),

         _ => NONE
      }
   }
}

trait BBoxTrait<T1> {
   fn bbox(object: T1) -> Data;
}

impl BBoxTrait<T2I64> for T2I64 {
   #[inline]
   fn bbox(object: T2I64) -> Data {
      Data::T2T2I64(
         Box::new(
            ((object.0, object.1), (object.0, object.1))
         )
      )
   }
}

impl BBoxTrait<VT2I64> for VT2I64 {
   #[inline]
   fn bbox(object: VT2I64) -> Data {
      if object.len() == 0 {
         return NONE;
      }

      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for tuple in object.iter() {
         if tuple.0 < left {
            left = tuple.0;
         }

         if tuple.0 > right {
            right = tuple.0;
         }

         if tuple.1 < top {
            top = tuple.1;
         }

         if tuple.1 > bottom {
            bottom = tuple.1;
         }
      }

      Data::T2T2I64(
         Box::new(
            ((left, top), (right, bottom))
         )
      )
   }
}

impl BBoxTrait<VVT2I64> for VVT2I64 {
   #[inline]
   fn bbox(object: VVT2I64) -> Data {
      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for outer in &object {
         for tuple in outer.iter() {
            if tuple.0 < left {
               left = tuple.0;
            }

            if tuple.0 > right {
               right = tuple.0;
            }

            if tuple.1 < top {
               top = tuple.1;
            }

            if tuple.1 > bottom {
               bottom = tuple.1;
            }
         }
      }

      if top == i64::MAX {
         return NONE;
      }

      Data::T2T2I64(
         Box::new(
            ((left, top), (right, bottom))
         )
      )
   }
}


#[derive(Debug)]
pub struct Center { }

impl Operator for Center {
   #[inline]
   fn new() -> Self {
      Center { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in_object = node.input(state, 0);

      match in_object {
         Data::T2I64(object) => <T2I64>::center(object),

         Data::T2T2I64(object) => <T2T2I64>::center(*object),

         Data::VT2I64(object) => <VT2I64>::center(object),

         Data::VVT2I64(object) => <VVT2I64>::center(object),

         _ => NONE
      }
   }
}

trait CenterTrait<T1> {
   fn center(object: T1) -> Data;
}

impl CenterTrait<T2I64> for T2I64 {
   #[inline]
   fn center(object: T2I64) -> Data {
      Data::T2I64(object)
   }
}

impl CenterTrait<T2T2I64> for T2T2I64 {
   #[inline]
   fn center(object: T2T2I64) -> Data {
      let x = ((object.0).0 + (object.1).0) / 2;
      let y = ((object.0).1 + (object.1).1) / 2;
      Data::T2I64((x, y))
   }
}

impl CenterTrait<VT2I64> for VT2I64 {
   #[inline]
   fn center(object: VT2I64) -> Data {
      let bbox = <VT2I64>::bbox(object);

      match bbox {
         Data::T2T2I64(bbox) => <T2T2I64>::center(*bbox),
         _ => NONE,
      }
   }
}

impl CenterTrait<VVT2I64> for VVT2I64 {
   #[inline]
   fn center(object: VVT2I64) -> Data {
      let bbox = <VVT2I64>::bbox(object);

      match bbox {
         Data::T2T2I64(bbox) => <T2T2I64>::center(*bbox),
         _ => NONE,
      }
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
         (Data::I64(v1), Data::I64(v2)) => Data::T2I64((v1, v2)),
         _ => NONE
      }
   }

   #[inline]
   fn process_three(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);
      let in3 = node.input(state, 2);

      match (in1, in2, in3) {
         (Data::U8(v1), Data::U8(v2), Data::U8(v3)) => Data::T3U8((v1, v2, v3)),
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
         (Data::VT2I64(v1), Data::T3U8(v2)) => <(VT2I64, T3U8)>::build_poly(v1, v2),
         _ => NONE
      }
   }
}

trait BuildPolyTrait<T1, T2> {
   fn build_poly(v1: T1, v2: T2) -> Data;
}

impl BuildPolyTrait<VT2I64, T3U8> for (VT2I64, T3U8) {
   #[inline]
   fn build_poly(array: VT2I64, color: T3U8) -> Data {
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


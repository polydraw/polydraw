use std::fmt;
use std::i64;

use draw::RGB;

use super::node::{Node, NodeRole};
use super::data::{Data, NONE, Layer, Point, PointList, Poly, PointListList, Rect};
use super::builder::Program;


pub trait Operator where Self: fmt::Debug {
   fn process(&self, program: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data>;

   fn role(&self) -> NodeRole {
      NodeRole::Processor
   }
}


#[derive(Debug)]
pub struct NoneOperator { }

impl NoneOperator {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(NoneOperator {})
   }
}

impl Operator for NoneOperator {
   #[inline]
   fn process(&self, _: &mut Program, _: &Node, _: &mut [Vec<Data>]) -> Option<Data> {
      Some(NONE)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct InputOperator { }

impl InputOperator {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(InputOperator {})
   }
}

impl Operator for InputOperator {
   #[inline]
   fn process(&self, _: &mut Program, _: &Node, _: &mut [Vec<Data>]) -> Option<Data> {
      None
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct DataOperator { }

impl DataOperator {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(DataOperator {})
   }
}

impl Operator for DataOperator {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      Some(node.input(state, 0))
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct SourceOperator { }

impl SourceOperator {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(SourceOperator {})
   }
}

impl Operator for SourceOperator {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      Some(node.input(state, 0))
   }
}


#[derive(Debug)]
pub struct Print { }

impl Print {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Print {})
   }
}

impl Operator for Print {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      match (in1, in2) {
         (Data::Int(frame), Data::Int(target)) => {
            if frame == target {
               for i in 2..node.len() {
                  let input = node.input(state, i);

                  println!("[{}] {:?}", i-2, input);
               }
            }
         },

         _ => {}
      }

      None
   }
}


#[derive(Debug)]
pub struct Add { }

impl Add {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Add {})
   }
}

impl Operator for Add {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_add(in1, in2))
   }
}

pub fn eval_add(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.add(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.add(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.add(v1),

      (Data::Point(v1), Data::Int(v2)) => v1.add(v2),
      (Data::Int(v1), Data::Point(v2)) => v2.add(v1),

      (Data::Point(v1), Data::Point(v2)) => v1.add(v2),

      (Data::PointList(v1), Data::Point(v2)) => v1.add(v2),
      (Data::Point(v1), Data::PointList(v2)) => v2.add(v1),

      (Data::PointListList(v1), Data::Point(v2)) => v1.add(v2),
      (Data::Point(v1), Data::PointListList(v2)) => v2.add(v1),

      _ => NONE
   }
}

trait AddTrait<Rhs> {
   fn add(self, v2: Rhs) -> Data;
}

impl AddTrait<i64> for i64 {
   #[inline]
   fn add(self, v2: i64) -> Data {
      Data::Int(self + v2)
   }
}

impl AddTrait<i64> for f64 {
   #[inline]
   fn add(self, v2: i64) -> Data {
      Data::Float(self + v2 as f64)
   }
}

impl AddTrait<i64> for Point {
   #[inline]
   fn add(mut self, v2: i64) -> Data {
      self.x += v2;
      self.y += v2;

      Data::Point(self)
   }
}

impl AddTrait<Point> for Point {
   #[inline]
   fn add(mut self, v2: Point) -> Data {
      self.x += v2.x;
      self.y += v2.y;

      Data::Point(self)
   }
}

impl AddTrait<Point> for Box<PointList> {
   #[inline]
   fn add(mut self, v2: Point) -> Data {
      for point in self.iter_mut() {
         point.x += v2.x;
         point.y += v2.y;
      }

      Data::PointList(self)
   }
}

impl AddTrait<Point> for Box<PointListList> {
   #[inline]
   fn add(mut self, v2: Point) -> Data {
      for src in self.iter_mut() {
         for point in src.iter_mut() {
            point.x += v2.x;
            point.y += v2.y;
         }
      }

      Data::PointListList(self)
   }
}


#[derive(Debug)]
pub struct Divide { }

impl Divide {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Divide {})
   }
}

impl Operator for Divide {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_divide(in1, in2))
   }
}

pub fn eval_divide(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.divide(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.divide(v2),
      (Data::Int(v1), Data::Float(v2)) => v1.divide(v2),

      (Data::Point(v1), Data::Int(v2)) => v1.divide(v2),

      (Data::Point(v1), Data::Point(v2)) => v1.divide(v2),

      (Data::PointList(v1), Data::Point(v2)) => v1.divide(v2),

      (Data::PointListList(v1), Data::Point(v2)) => v1.divide(v2),

      _ => NONE
   }
}

trait DivideTrait<Rhs> {
   fn divide(self, v2: Rhs) -> Data;
}

impl DivideTrait<i64> for i64 {
   #[inline]
   fn divide(self, v2: i64) -> Data {
      Data::Int(self / v2)
   }
}

impl DivideTrait<i64> for f64 {
   #[inline]
   fn divide(self, v2: i64) -> Data {
      Data::Float(self / v2 as f64)
   }
}

impl DivideTrait<f64> for i64 {
   #[inline]
   fn divide(self, v2: f64) -> Data {
      Data::Float(self as f64 / v2)
   }
}

impl DivideTrait<i64> for Point {
   #[inline]
   fn divide(mut self, v2: i64) -> Data {
      self.x /= v2;
      self.y /= v2;

      Data::Point(self)
   }
}

impl DivideTrait<Point> for Point {
   #[inline]
   fn divide(mut self, v2: Point) -> Data {
      self.x /= v2.x;
      self.y /= v2.y;

      Data::Point(self)
   }
}

impl DivideTrait<Point> for Box<PointList> {
   #[inline]
   fn divide(mut self, v2: Point) -> Data {
      for point in self.iter_mut() {
         point.x /= v2.x;
         point.y /= v2.y;
      }

      Data::PointList(self)
   }
}

impl DivideTrait<Point> for Box<PointListList> {
   #[inline]
   fn divide(mut self, v2: Point) -> Data {
      for src in self.iter_mut() {
         for point in src.iter_mut() {
            point.x /= v2.x;
            point.y /= v2.y;
         }
      }

      Data::PointListList(self)
   }
}


#[derive(Debug)]
pub struct Subtract { }

impl Subtract {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Subtract {})
   }
}

impl Operator for Subtract {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_subtract(in1, in2))
   }
}

pub fn eval_subtract(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.subtract(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.subtract(v2),
      (Data::Int(v1), Data::Float(v2)) => v1.subtract(v2),

      (Data::Point(v1), Data::Int(v2)) => v1.subtract(v2),

      (Data::Point(v1), Data::Point(v2)) => v1.subtract(v2),

      (Data::PointList(v1), Data::Point(v2)) => v1.subtract(v2),

      (Data::PointListList(v1), Data::Point(v2)) => v1.subtract(v2),

      _ => NONE
   }
}

trait SubtractTrait<Rhs> {
   fn subtract(self, v2: Rhs) -> Data;
}

impl SubtractTrait<i64> for i64 {
   #[inline]
   fn subtract(self, v2: i64) -> Data {
      Data::Int(self - v2)
   }
}

impl SubtractTrait<i64> for f64 {
   #[inline]
   fn subtract(self, v2: i64) -> Data {
      Data::Float(self - v2 as f64)
   }
}

impl SubtractTrait<f64> for i64 {
   #[inline]
   fn subtract(self, v2: f64) -> Data {
      Data::Float(self as f64 - v2)
   }
}

impl SubtractTrait<i64> for Point {
   #[inline]
   fn subtract(mut self, v2: i64) -> Data {
      self.x -= v2;
      self.y -= v2;

      Data::Point(self)
   }
}

impl SubtractTrait<Point> for Point {
   #[inline]
   fn subtract(mut self, v2: Point) -> Data {
      self.x -= v2.x;
      self.y -= v2.y;

      Data::Point(self)
   }
}

impl SubtractTrait<Point> for Box<PointList> {
   #[inline]
   fn subtract(mut self, v2: Point) -> Data {
      for point in self.iter_mut() {
         point.x -= v2.x;
         point.y -= v2.y;
      }

      Data::PointList(self)
   }
}

impl SubtractTrait<Point> for Box<PointListList> {
   #[inline]
   fn subtract(mut self, v2: Point) -> Data {
      for src in self.iter_mut() {
         for point in src.iter_mut() {
            point.x -= v2.x;
            point.y -= v2.y;
         }
      }

      Data::PointListList(self)
   }
}


#[derive(Debug)]
pub struct Multiply { }

impl Multiply {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Multiply {})
   }
}

impl Operator for Multiply {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_multiply(in1, in2))
   }
}

pub fn eval_multiply(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.multiply(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.multiply(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.multiply(v1),

      (Data::Point(v1), Data::Int(v2)) => v1.multiply(v2),
      (Data::Int(v1), Data::Point(v2)) => v2.multiply(v1),

      (Data::Point(v1), Data::Point(v2)) => v1.multiply(v2),

      (Data::PointList(v1), Data::Point(v2)) => v1.multiply(v2),
      (Data::Point(v1), Data::PointList(v2)) => v2.multiply(v1),

      (Data::PointListList(v1), Data::Point(v2)) => v1.multiply(v2),
      (Data::Point(v1), Data::PointListList(v2)) => v2.multiply(v1),

      _ => NONE
   }
}

trait MultiplyTrait<Rhs> {
   fn multiply(self, v2: Rhs) -> Data;
}

impl MultiplyTrait<i64> for i64 {
   #[inline]
   fn multiply(self, v2: i64) -> Data {
      Data::Int(self * v2)
   }
}

impl MultiplyTrait<i64> for f64 {
   #[inline]
   fn multiply(self, v2: i64) -> Data {
      Data::Float(self * v2 as f64)
   }
}

impl MultiplyTrait<i64> for Point {
   #[inline]
   fn multiply(mut self, v2: i64) -> Data {
      self.x *= v2;
      self.y *= v2;

      Data::Point(self)
   }
}

impl MultiplyTrait<Point> for Point {
   #[inline]
   fn multiply(mut self, v2: Point) -> Data {
      self.x *= v2.x;
      self.y *= v2.y;

      Data::Point(self)
   }
}

impl MultiplyTrait<Point> for Box<PointList> {
   #[inline]
   fn multiply(mut self, v2: Point) -> Data {
      for point in self.iter_mut() {
         point.x *= v2.x;
         point.y *= v2.y;
      }

      Data::PointList(self)
   }
}

impl MultiplyTrait<Point> for Box<PointListList> {
   #[inline]
   fn multiply(mut self, v2: Point) -> Data {
      for src in self.iter_mut() {
         for point in src.iter_mut() {
            point.x *= v2.x;
            point.y *= v2.y;
         }
      }

      Data::PointListList(self)
   }
}


#[derive(Debug)]
pub struct Equal { }

impl Equal {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Equal {})
   }
}

impl Operator for Equal {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_equal(in1, in2))
   }
}

pub fn eval_equal(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.equal(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.equal(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.equal(v1),

      _ => NONE
   }
}

pub trait EqualTrait<Rhs> {
   fn equal(self, other: Rhs) -> Data;
}

impl EqualTrait<i64> for i64 {
   #[inline]
   fn equal(self, other: i64) -> Data {
      Data::Bool(self == other)
   }
}

impl EqualTrait<i64> for f64 {
   #[inline]
   fn equal(self, other: i64) -> Data {
      Data::Bool(self == other as f64)
   }
}


#[derive(Debug)]
pub struct Unequal { }

impl Unequal {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Unequal {})
   }
}

impl Operator for Unequal {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_unequal(in1, in2))
   }
}

pub fn eval_unequal(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.unequal(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.unequal(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.unequal(v1),

      _ => NONE
   }
}

pub trait UnequalTrait<Rhs> {
   fn unequal(self, other: Rhs) -> Data;
}

impl UnequalTrait<i64> for i64 {
   #[inline]
   fn unequal(self, other: i64) -> Data {
      Data::Bool(self != other)
   }
}

impl UnequalTrait<i64> for f64 {
   #[inline]
   fn unequal(self, other: i64) -> Data {
      Data::Bool(self != other as f64)
   }
}


#[derive(Debug)]
pub struct Less { }

impl Less {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Less {})
   }
}

impl Operator for Less {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_less(in1, in2))
   }
}

pub fn eval_less(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.less(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.less(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.less(v1),

      _ => NONE
   }
}

pub trait LessTrait<Rhs> {
   fn less(self, other: Rhs) -> Data;
}

impl LessTrait<i64> for i64 {
   #[inline]
   fn less(self, other: i64) -> Data {
      Data::Bool(self < other)
   }
}

impl LessTrait<i64> for f64 {
   #[inline]
   fn less(self, other: i64) -> Data {
      Data::Bool(self < other as f64)
   }
}


#[derive(Debug)]
pub struct LessEqual { }

impl LessEqual {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(LessEqual {})
   }
}

impl Operator for LessEqual {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_less_equal(in1, in2))
   }
}

pub fn eval_less_equal(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.less_equal(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.less_equal(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.less_equal(v1),

      _ => NONE
   }
}

pub trait LessEqualTrait<Rhs> {
   fn less_equal(self, other: Rhs) -> Data;
}

impl LessEqualTrait<i64> for i64 {
   #[inline]
   fn less_equal(self, other: i64) -> Data {
      Data::Bool(self <= other)
   }
}

impl LessEqualTrait<i64> for f64 {
   #[inline]
   fn less_equal(self, other: i64) -> Data {
      Data::Bool(self <= other as f64)
   }
}


#[derive(Debug)]
pub struct Greater { }

impl Greater {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Greater {})
   }
}

impl Operator for Greater {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_greater(in1, in2))
   }
}

pub fn eval_greater(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.greater(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.greater(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.greater(v1),

      _ => NONE
   }
}

pub trait GreaterTrait<Rhs> {
   fn greater(self, other: Rhs) -> Data;
}

impl GreaterTrait<i64> for i64 {
   #[inline]
   fn greater(self, other: i64) -> Data {
      Data::Bool(self > other)
   }
}

impl GreaterTrait<i64> for f64 {
   #[inline]
   fn greater(self, other: i64) -> Data {
      Data::Bool(self > other as f64)
   }
}


#[derive(Debug)]
pub struct GreaterEqual { }

impl GreaterEqual {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(GreaterEqual {})
   }
}

impl Operator for GreaterEqual {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_greater_equal(in1, in2))
   }
}

pub fn eval_greater_equal(in1: Data, in2: Data) -> Data {
   match (in1, in2) {
      (Data::Int(v1), Data::Int(v2)) => v1.greater_equal(v2),

      (Data::Float(v1), Data::Int(v2)) => v1.greater_equal(v2),
      (Data::Int(v1), Data::Float(v2)) => v2.greater_equal(v1),

      _ => NONE
   }
}

pub trait GreaterEqualTrait<Rhs> {
   fn greater_equal(self, other: Rhs) -> Data;
}

impl GreaterEqualTrait<i64> for i64 {
   #[inline]
   fn greater_equal(self, other: i64) -> Data {
      Data::Bool(self >= other)
   }
}

impl GreaterEqualTrait<i64> for f64 {
   #[inline]
   fn greater_equal(self, other: i64) -> Data {
      Data::Bool(self >= other as f64)
   }
}


#[derive(Debug)]
pub struct Nth { }

impl Nth {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Nth {})
   }
}

impl Operator for Nth {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      let result = match (in1, in2) {
         (Data::Rgb(object), Data::Int(index)) => <RGB>::nth(object, index),

         (Data::Point(object), Data::Int(index)) => <Point>::nth(object, index),

         (Data::Rect(object), Data::Int(index)) => <Box<Rect>>::nth(object, index),

//         (Data::PointList(object), Data::Int(index)) => <PointList>::nth(object, index),

//         (Data::PointListList(object), Data::Int(index)) => <PointListList>::nth(object, index),

         _ => NONE
      };

      Some(result)
   }
}

trait NthTrait {
   fn nth(self, index: i64) -> Data;
}

impl NthTrait for Point {
   #[inline]
   fn nth(self, index: i64) -> Data {
      match index {
         0 => Data::Int(self.x),
         1 => Data::Int(self.y),
         _ => NONE,
      }
   }
}

impl NthTrait for RGB {
   #[inline]
   fn nth(self, index: i64) -> Data {
      match index {
         0 => Data::Int(self.r as i64),
         1 => Data::Int(self.g as i64),
         2 => Data::Int(self.b as i64),
         _ => NONE,
      }
   }
}


impl NthTrait for Box<Rect> {
   #[inline]
   fn nth(self, index: i64) -> Data {
      match index {
         0 => Data::Point(self.p1),
         1 => Data::Point(self.p2),
         _ => NONE,
      }
   }
}

#[derive(Debug)]
pub struct Polar { }

impl Polar {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Polar {})
   }
}

impl Operator for Polar {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let radius = node.input(state, 0);
      let angle = node.input(state, 1);

      Some(eval_polar(radius, angle))
   }
}

pub fn eval_polar(radius: Data, angle: Data) -> Data {
   match (radius, angle) {
      (Data::Float(radius), Data::Float(angle)) => <(f64, f64)>::polar(radius, angle),
      (Data::Int(radius), Data::Float(angle)) => <(f64, f64)>::polar(radius as f64, angle),
      (Data::Float(radius), Data::Int(angle)) => <(f64, f64)>::polar(radius, angle as f64),
      (Data::Int(radius), Data::Int(angle)) => <(f64, f64)>::polar(radius as f64, angle as f64),
      _ => NONE
   }
}


trait PolarTrait<T1, T2> {
   fn polar(radius: T1, angle: T2) -> Data;
}

impl PolarTrait<f64, f64> for (f64, f64) {
   #[inline]
   fn polar(radius: f64, angle: f64) -> Data {
      let radians = angle.to_radians();

      let x = radius * radians.cos();
      let y = radius * radians.sin();

      Data::Point(Point::new(x as i64, y as i64))
   }
}


#[derive(Debug)]
pub struct Rotate { }

impl Rotate {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Rotate {})
   }
}

impl Operator for Rotate {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let target = node.input(state, 0);
      let origin = node.input(state, 1);
      let angle = node.input(state, 2);

      Some(eval_rotate(target, origin, angle))
   }
}

pub fn eval_rotate(target: Data, origin: Data, angle: Data) -> Data {
   match (target, origin, angle) {
      (Data::Point(target), Data::Point(origin), Data::Float(angle)) =>
         target.rotate(origin, angle),

      (Data::Point(target), Data::Point(origin), Data::Int(angle)) =>
         target.rotate(origin, angle),

      (Data::PointList(target), Data::Point(origin), Data::Float(angle)) =>
         target.rotate(origin, angle),

      (Data::PointList(target), Data::Point(origin), Data::Int(angle)) =>
         target.rotate(origin, angle),

      (Data::PointListList(target), Data::Point(origin), Data::Float(angle)) =>
         target.rotate(origin, angle),

      (Data::PointListList(target), Data::Point(origin), Data::Int(angle)) =>
         target.rotate(origin, angle),

      _ => NONE
   }
}

trait RotateTrait<T2> {
   fn rotate(self, origin: Point, angle: T2) -> Data;
}

impl RotateTrait<f64> for Point {
   #[inline]
   fn rotate(mut self, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      let mut x = self.x as f64;
      let mut y = self.y as f64;

      x -= cx;
      y -= cy;

      self.x = (x * c - y * s + cx) as i64;
      self.y = (x * s + y * c + cy) as i64;

      Data::Point(self)
   }
}

impl RotateTrait<i64> for Point {
   #[inline]
   fn rotate(self, origin: Point, angle: i64) -> Data {
      self.rotate(origin, angle as f64)
   }
}

impl RotateTrait<f64> for Box<PointList> {
   #[inline]
   fn rotate(mut self, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for tuple in self.iter_mut() {
         let mut x = tuple.x as f64;
         let mut y = tuple.y as f64;

         x -= cx;
         y -= cy;

         tuple.x = (x * c - y * s + cx) as i64;
         tuple.y = (x * s + y * c + cy) as i64;
      }

      Data::PointList(self)
   }
}

impl RotateTrait<i64> for Box<PointList> {
   #[inline]
   fn rotate(self, origin: Point, angle: i64) -> Data {
      self.rotate(origin, angle as f64)
   }
}

impl RotateTrait<f64> for Box<PointListList> {
   #[inline]
   fn rotate(mut self, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for outer in self.iter_mut() {
         for tuple in outer.iter_mut() {
            let mut x = tuple.x as f64;
            let mut y = tuple.y as f64;

            x -= cx;
            y -= cy;

            tuple.x = (x * c - y * s + cx) as i64;
            tuple.y = (x * s + y * c + cy) as i64;
         }
      }

      Data::PointListList(self)
   }
}

impl RotateTrait<i64> for Box<PointListList> {
   #[inline]
   fn rotate(self, origin: Point, angle: i64) -> Data {
      self.rotate(origin, angle as f64)
   }
}


#[derive(Debug)]
pub struct BBox { }

impl BBox {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BBox {})
   }
}

impl Operator for BBox {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let object = node.input(state, 0);

      Some(eval_bbox(object))
   }
}

pub fn eval_bbox(object: Data) -> Data {
   match object {
      Data::Point(object) => object.bbox(),

      Data::PointList(object) => object.bbox(),

      Data::PointListList(object) => object.bbox(),

      _ => NONE
   }
}

trait BBoxTrait {
   fn bbox(self) -> Data;
}

impl BBoxTrait for Point {
   #[inline]
   fn bbox(self) -> Data {
      Data::Rect(
         Box::new(
            Rect::new(self, self)
         )
      )
   }
}

impl BBoxTrait for Box<PointList> {
   #[inline]
   fn bbox(self) -> Data {
      if self.len() == 0 {
         return NONE;
      }

      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for tuple in self.iter() {
         if tuple.x < left {
            left = tuple.x;
         }

         if tuple.x > right {
            right = tuple.x;
         }

         if tuple.y < top {
            top = tuple.y;
         }

         if tuple.y > bottom {
            bottom = tuple.y;
         }
      }

      Data::Rect(
         Box::new(
            Rect::new(
               Point::new(left, top), Point::new(right, bottom)
            )
         )
      )
   }
}

impl BBoxTrait for Box<PointListList> {
   #[inline]
   fn bbox(self) -> Data {
      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for outer in self.iter() {
         for tuple in outer.iter() {
            if tuple.x < left {
               left = tuple.x;
            }

            if tuple.x > right {
               right = tuple.x;
            }

            if tuple.y < top {
               top = tuple.y;
            }

            if tuple.y > bottom {
               bottom = tuple.y;
            }
         }
      }

      if top == i64::MAX {
         return NONE;
      }

      Data::Rect(
         Box::new(
            Rect::new(
               Point::new(left, top), Point::new(right, bottom)
            )
         )
      )
   }
}


#[derive(Debug)]
pub struct Center { }

impl Center {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Center {})
   }
}

impl Operator for Center {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let object = node.input(state, 0);

      Some(eval_center(object))
   }
}

pub fn eval_center(object: Data) -> Data {
   match object {
      Data::Point(object) => object.center(),
      Data::Rect(object) => object.center(),
      Data::PointList(object) => object.center(),
      Data::PointListList(object) => object.center(),
      _ => NONE
   }
}

trait CenterTrait {
   fn center(self) -> Data;
}

impl CenterTrait for Point {
   #[inline]
   fn center(self) -> Data {
      Data::Point(self)
   }
}

impl CenterTrait for Rect {
   #[inline]
   fn center(self) -> Data {
      let x = (self.p1.x + self.p2.x) / 2;
      let y = (self.p1.y + self.p1.y) / 2;
      Data::Point(Point::new(x, y))
   }
}

impl CenterTrait for Box<PointList> {
   #[inline]
   fn center(self) -> Data {
      let bbox = self.bbox();

      match bbox {
         Data::Rect(bbox) => bbox.center(),
         _ => NONE,
      }
   }
}

impl CenterTrait for Box<PointListList> {
   #[inline]
   fn center(self) -> Data {
      let bbox = self.bbox();

      match bbox {
         Data::Rect(bbox) => bbox.center(),
         _ => NONE,
      }
   }
}


#[derive(Debug)]
pub struct BuildPoint { }

impl BuildPoint {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildPoint {})
   }
}

impl Operator for BuildPoint {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      let result = match (in1, in2) {
         (Data::Int(v1), Data::Int(v2)) => Data::Point(Point::new(v1, v2)),
         _ => NONE
      };

      Some(result)
   }
}


#[derive(Debug)]
pub struct BuildRgb { }

impl BuildRgb {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildRgb {})
   }
}

impl Operator for BuildRgb {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let red = node.input(state, 0);
      let green = node.input(state, 1);
      let blue = node.input(state, 2);

      Some(eval_rgb(red, green, blue))
   }
}

pub fn eval_rgb(red: Data, green: Data, blue: Data) -> Data {
   match (red, green, blue) {
      (Data::Int(red), Data::Int(green), Data::Int(blue)) =>
         Data::Rgb(RGB::new(red as u8, green as u8, blue as u8)),
      _ => NONE
   }
}

#[derive(Debug)]
pub struct BuildList { }

impl BuildList {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildList {})
   }
}

impl Operator for BuildList {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let first = node.input(state, 0);

      let result = match first {
         Data::Int(first) => first.list(node, state),
         Data::Float(first) => first.list(node, state),
         Data::Bool(first) => first.list(node, state),
         Data::Point(first) => first.list(node, state),
         Data::Rgb(first) => first.list(node, state),
         Data::Poly(first) => first.list(node, state),
         Data::Layer(first) => first.list(node, state),
         _ => NONE
      };

      Some(result)
   }
}

trait BuildListTrait {
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data;
}

impl BuildListTrait for i64 {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Int(value) = input {
            result.push(value);
         }
      }

      Data::IntList(Box::new(result))
   }
}

impl BuildListTrait for f64 {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Float(value) = input {
            result.push(value);
         }
      }

      Data::FloatList(Box::new(result))
   }
}

impl BuildListTrait for bool {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Bool(value) = input {
            result.push(value);
         }
      }

      Data::BoolList(Box::new(result))
   }
}

impl BuildListTrait for Point {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Point(value) = input {
            result.push(value);
         }
      }

      Data::PointList(Box::new(result))
   }
}

impl BuildListTrait for RGB {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Rgb(value) = input {
            result.push(value);
         }
      }

      Data::RgbList(Box::new(result))
   }
}

impl BuildListTrait for Box<Poly> {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(*self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Poly(value) = input {
            result.push(*value);
         }
      }

      Data::PolyList(Box::new(result))
   }
}

impl BuildListTrait for Box<Layer> {
   #[inline]
   fn list(self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(*self);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Layer(value) = input {
            result.push(*value);
         }
      }

      Data::LayerList(Box::new(result))
   }
}


#[derive(Debug)]
pub struct BuildPoly { }

impl BuildPoly {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildPoly {})
   }
}

impl Operator for BuildPoly {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let points = node.input(state, 0);
      let color = node.input(state, 1);

      let result = match (points, color) {
         (Data::PointList(v1), Data::Rgb(v2)) => v1.build_poly(v2),
         _ => NONE
      };

      Some(result)
   }
}

trait BuildPolyTrait {
   fn build_poly(self, color: RGB) -> Data;
}

impl BuildPolyTrait for Box<PointList> {
   #[inline]
   fn build_poly(self, color: RGB) -> Data {
      let color = RGB::new(color.r, color.g, color.b);

      let poly = Poly::new(*self, color);

      Data::Poly(Box::new(poly))
   }
}


#[derive(Debug)]
pub struct BuildLayer { }

impl BuildLayer {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildLayer {})
   }
}

impl Operator for BuildLayer {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let polys_data = node.input(state, 0);

      let result = match polys_data {
         Data::PolyList(polys) => {
            Data::Layer(
               Box::new(
                  Layer::new(*polys)
               )
            )
         },
         _ => NONE
      };

      Some(result)
   }
}


#[derive(Debug)]
pub struct BuildArtboard {
   list_node: BuildList,
}

impl BuildArtboard {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(BuildArtboard {
         list_node: *BuildList::new()
      })
   }
}

impl Operator for BuildArtboard {
   #[inline]
   fn process(&self, program: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      self.list_node.process(program, node, state)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Artboard
   }
}


#[derive(Debug)]
pub struct Gate { }

impl Gate {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Gate {})
   }
}

impl Operator for Gate {
   #[inline]
   fn process(&self, _: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      Some(eval_gate(in1, in2))
   }
}

pub fn eval_gate(in1: Data, in2: Data) -> Data {
   match in2 {
      Data::Bool(in2) => {
         if in2 {
            in1
         } else {
            NONE
         }
      },
      _ => NONE
   }
}


#[derive(Debug)]
pub struct FunctionOperator {
   name: String,
}

impl FunctionOperator {
   #[inline]
   pub fn new(name: String) -> Box<Self> {
      Box::new(
         FunctionOperator {
            name: name
         }
      )
   }
}

impl Operator for FunctionOperator {
   #[inline]
   fn process(
      &self, program: &mut Program, node: &Node, state: &mut [Vec<Data>]
   ) -> Option<Data> {
      match program.argument_count(&self.name) {
         Some(count) => {
            let mut arguments = Vec::with_capacity(count);

            for i in 0..count {
               arguments.push(node.input(state, i));
            }

            Some(program.execute_function(self.name.clone(), arguments))
         },
         None => None,
      }
   }
}


#[derive(Debug)]
pub struct Each { }

impl Each {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(Each {})
   }
}

impl Operator for Each {
   #[inline]
   fn process(&self, program: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let target = node.input(state, 0);
      let function = node.input(state, 1);

      let data = if let Data::FunctionRef(function) = function {
         if let Some(count) = program.argument_count(&function) {
            let mut extra = Vec::new();

            for i in 2..count + 1 {
               extra.push(node.input(state, i));
            }

            match target {
               Data::IntList(list) => list.each(program, function, extra),
               Data::FloatList(list) => list.each(program, function, extra),
               Data::BoolList(list) => list.each(program, function, extra),
               Data::PointList(list) => list.each(program, function, extra),
               Data::RgbList(list) => list.each(program, function, extra),
               _ => NONE,
            }
         } else {
            NONE
         }
      } else {
         NONE
      };

      Some(data)
   }
}

trait EachTrait {
   fn each(self, program: &mut Program, function: String, extra: Vec<Data>) -> Data;
}

macro_rules! each_trait {
   ($trait_ty:ty, $data_ty:expr) => {
      impl EachTrait for $trait_ty {
         #[inline]
         fn each(self, program: &mut Program, function: String, extra: Vec<Data>) -> Data {
            let mut list = Vec::new();

            for value in self {
               let mut arguments = vec![$data_ty(value)];
               arguments.append(&mut extra.clone());

               let data = program.execute_function(function.clone(), arguments);
               list.push(data);
            }

            Data::DataList(Box::new(list))
         }
      }
   }
}

each_trait!(Vec<i64>, Data::Int);
each_trait!(Vec<f64>, Data::Float);
each_trait!(Vec<bool>, Data::Bool);
each_trait!(Vec<Point>, Data::Point);
each_trait!(Vec<RGB>, Data::Rgb);


#[derive(Debug)]
pub struct EachWithLast { }

impl EachWithLast {
   #[inline]
   pub fn new() -> Box<Self> {
      Box::new(EachWithLast {})
   }
}

impl Operator for EachWithLast {
   #[inline]
   fn process(&self, program: &mut Program, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let target = node.input(state, 0);
      let function = node.input(state, 1);
      let initial = node.input(state, 2);

      let data = if let Data::FunctionRef(function) = function {
         if let Some(count) = program.argument_count(&function) {
            let mut extra = Vec::new();

            for i in 3..count + 1 {
               extra.push(node.input(state, i));
            }

            match target {
               Data::IntList(list) => list.each_with_last(program, function, initial, extra),
               Data::FloatList(list) => list.each_with_last(program, function, initial, extra),
               Data::BoolList(list) => list.each_with_last(program, function, initial, extra),
               Data::PointList(list) => list.each_with_last(program, function, initial, extra),
               Data::RgbList(list) => list.each_with_last(program, function, initial, extra),
               _ => NONE,
            }
         } else {
            NONE
         }
      } else {
         NONE
      };

      Some(data)
   }
}

trait EachWithLastTrait {
   fn each_with_last(
      self, program: &mut Program, function: String, initial: Data, extra: Vec<Data>
   ) -> Data;
}

macro_rules! each_with_last_trait {
   ($trait_ty:ty, $data_ty:expr) => {
      impl EachWithLastTrait for $trait_ty {
         #[inline]
         fn each_with_last(
            self, program: &mut Program, function: String, mut initial: Data, extra: Vec<Data>
         ) -> Data {
            let mut list = Vec::new();

            for value in self {
               let mut arguments = vec![$data_ty(value), initial];
               arguments.append(&mut extra.clone());

               let data = program.execute_function(function.clone(), arguments);
               initial = data.clone();
               list.push(data);
            }

            Data::DataList(Box::new(list))
         }
      }
   }
}

each_with_last_trait!(Vec<i64>, Data::Int);
each_with_last_trait!(Vec<f64>, Data::Float);
each_with_last_trait!(Vec<bool>, Data::Bool);
each_with_last_trait!(Vec<Point>, Data::Point);
each_with_last_trait!(Vec<RGB>, Data::Rgb);

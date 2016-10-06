use std::fmt;
use std::i64;

use draw::RGB;

use super::node::{Node, NodeRole};
use super::data::{Data, NONE, Layer, Point, PointList, Poly, PointListList, BBox};


pub trait Operator where Self: fmt::Debug {
   fn new() -> Self where Self: Sized;

   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data>;

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
   fn process(&self, _: &Node, _: &mut [Vec<Data>]) -> Option<Data> {
      Some(NONE)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct InputOperator { }

impl Operator for InputOperator {
   #[inline]
   fn new() -> Self {
      InputOperator { }
   }

   #[inline]
   fn process(&self, _: &Node, _: &mut [Vec<Data>]) -> Option<Data> {
      None
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
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      Some(node.input(state, 0))
   }

   fn role(&self) -> NodeRole {
      NodeRole::Data
   }
}


#[derive(Debug)]
pub struct SourceOperator { }

impl Operator for SourceOperator {
   #[inline]
   fn new() -> Self {
      SourceOperator { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      Some(node.input(state, 0))
   }
}


#[derive(Debug)]
pub struct Print { }

impl Operator for Print {
   #[inline]
   fn new() -> Self {
      Print { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for Add {
   #[inline]
   fn new() -> Self {
      Add { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for Divide {
   #[inline]
   fn new() -> Self {
      Divide { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for Subtract {
   #[inline]
   fn new() -> Self {
      Subtract { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for Multiply {
   #[inline]
   fn new() -> Self {
      Multiply { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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
pub struct Nth { }

impl Operator for Nth {
   #[inline]
   fn new() -> Self {
      Nth { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let in1 = node.input(state, 0);
      let in2 = node.input(state, 1);

      let result = match (in1, in2) {
         (Data::Rgb(object), Data::Int(index)) => <RGB>::nth(object, index),

         (Data::Point(object), Data::Int(index)) => <Point>::nth(object, index),

         (Data::BBox(object), Data::Int(index)) => <Box<BBox>>::nth(object, index),

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


impl NthTrait for Box<BBox> {
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
pub struct Rotate { }

impl Operator for Rotate {
   #[inline]
   fn new() -> Self {
      Rotate { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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
pub struct BuildBBox { }

impl Operator for BuildBBox {
   #[inline]
   fn new() -> Self {
      BuildBBox { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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
      Data::BBox(
         Box::new(
            BBox::new(self, self)
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

      Data::BBox(
         Box::new(
            BBox::new(
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

      Data::BBox(
         Box::new(
            BBox::new(
               Point::new(left, top), Point::new(right, bottom)
            )
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
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let object = node.input(state, 0);

      Some(eval_center(object))
   }
}

pub fn eval_center(object: Data) -> Data {
   match object {
      Data::Point(object) => object.center(),
      Data::BBox(object) => object.center(),
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

impl CenterTrait for BBox {
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
         Data::BBox(bbox) => bbox.center(),
         _ => NONE,
      }
   }
}

impl CenterTrait for Box<PointListList> {
   #[inline]
   fn center(self) -> Data {
      let bbox = self.bbox();

      match bbox {
         Data::BBox(bbox) => bbox.center(),
         _ => NONE,
      }
   }
}


#[derive(Debug)]
pub struct BuildPoint { }

impl Operator for BuildPoint {
   #[inline]
   fn new() -> Self {
      BuildPoint { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for BuildRgb {
   #[inline]
   fn new() -> Self {
      BuildRgb { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for BuildList {
   #[inline]
   fn new() -> Self {
      BuildList { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for BuildPoly {
   #[inline]
   fn new() -> Self {
      BuildPoly { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for BuildLayer {
   #[inline]
   fn new() -> Self {
      BuildLayer { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
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

impl Operator for BuildArtboard {
   #[inline]
   fn new() -> Self {
      BuildArtboard {
         list_node: BuildList::new()
      }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      self.list_node.process(node, state)
   }

   fn role(&self) -> NodeRole {
      NodeRole::Artboard
   }
}


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
      (Data::Int(v1), Data::Int(v2)) => <(i64, i64)>::add(v1, v2),

      (Data::Float(v1), Data::Int(v2)) => <(f64, i64)>::add(v1, v2),
      (Data::Int(v1), Data::Float(v2)) => <(f64, i64)>::add(v2, v1),

      (Data::Point(v1), Data::Int(v2)) => <(Point, i64)>::add(v1, v2),
      (Data::Int(v1), Data::Point(v2)) => <(Point, i64)>::add(v2, v1),

      (Data::Point(v1), Data::Point(v2)) => <(Point, Point)>::add(v1, v2),

      (Data::PointList(v1), Data::Point(v2)) => <(Box<PointList>, Point)>::add(v1, v2),
      (Data::Point(v1), Data::PointList(v2)) => <(Box<PointList>, Point)>::add(v2, v1),

      (Data::PointListList(v1), Data::Point(v2)) => <(Box<PointListList>, Point)>::add(v1, v2),
      (Data::Point(v1), Data::PointListList(v2)) => <(Box<PointListList>, Point)>::add(v2, v1),

      _ => NONE
   }
}

trait AddTrait<T1, T2> {
   fn add(v1: T1, v2: T2) -> Data;
}

impl AddTrait<i64, i64> for (i64, i64) {
   #[inline]
   fn add(v1: i64, v2: i64) -> Data {
      Data::Int(v1 + v2)
   }
}

impl AddTrait<f64, i64> for (f64, i64) {
   #[inline]
   fn add(v1: f64, v2: i64) -> Data {
      Data::Float(v1 + v2 as f64)
   }
}

impl AddTrait<Point, i64> for (Point, i64) {
   #[inline]
   fn add(mut v1: Point, v2: i64) -> Data {
      v1.x += v2;
      v1.y += v2;

      Data::Point(v1)
   }
}

impl AddTrait<Point, Point> for (Point, Point) {
   #[inline]
   fn add(mut v1: Point, v2: Point) -> Data {
      v1.x += v2.x;
      v1.y += v2.y;

      Data::Point(v1)
   }
}

impl AddTrait<Box<PointList>, Point> for (Box<PointList>, Point) {
   #[inline]
   fn add(mut v1: Box<PointList>, v2: Point) -> Data {
      for point in v1.iter_mut() {
         point.x += v2.x;
         point.y += v2.y;
      }

      Data::PointList(v1)
   }
}

impl AddTrait<Box<PointListList>, Point> for (Box<PointListList>, Point) {
   #[inline]
   fn add(mut v1: Box<PointListList>, v2: Point) -> Data {
      for src in v1.iter_mut() {
         for point in src.iter_mut() {
            point.x += v2.x;
            point.y += v2.y;
         }
      }

      Data::PointListList(v1)
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
      (Data::Int(v1), Data::Int(v2)) => <(i64, i64)>::divide(v1, v2),

      (Data::Float(v1), Data::Int(v2)) => <(f64, i64)>::divide(v1, v2),
      (Data::Int(v1), Data::Float(v2)) => <(i64, f64)>::divide(v1, v2),

      (Data::Point(v1), Data::Int(v2)) => <(Point, i64)>::divide(v1, v2),

      (Data::Point(v1), Data::Point(v2)) => <(Point, Point)>::divide(v1, v2),

      (Data::PointList(v1), Data::Point(v2)) => <(Box<PointList>, Point)>::divide(v1, v2),

      (Data::PointListList(v1), Data::Point(v2)) => <(Box<PointListList>, Point)>::divide(v1, v2),

      _ => NONE
   }
}

trait DivideTrait<T1, T2> {
   fn divide(v1: T1, v2: T2) -> Data;
}

impl DivideTrait<i64, i64> for (i64, i64) {
   #[inline]
   fn divide(v1: i64, v2: i64) -> Data {
      Data::Int(v1 / v2)
   }
}

impl DivideTrait<f64, i64> for (f64, i64) {
   #[inline]
   fn divide(v1: f64, v2: i64) -> Data {
      Data::Float(v1 / v2 as f64)
   }
}

impl DivideTrait<i64, f64> for (i64, f64) {
   #[inline]
   fn divide(v1: i64, v2: f64) -> Data {
      Data::Float(v1 as f64 / v2)
   }
}

impl DivideTrait<Point, i64> for (Point, i64) {
   #[inline]
   fn divide(mut v1: Point, v2: i64) -> Data {
      v1.x /= v2;
      v1.y /= v2;

      Data::Point(v1)
   }
}

impl DivideTrait<Point, Point> for (Point, Point) {
   #[inline]
   fn divide(mut v1: Point, v2: Point) -> Data {
      v1.x /= v2.x;
      v1.y /= v2.y;

      Data::Point(v1)
   }
}

impl DivideTrait<Box<PointList>, Point> for (Box<PointList>, Point) {
   #[inline]
   fn divide(mut v1: Box<PointList>, v2: Point) -> Data {
      for point in v1.iter_mut() {
         point.x /= v2.x;
         point.y /= v2.y;
      }

      Data::PointList(v1)
   }
}

impl DivideTrait<Box<PointListList>, Point> for (Box<PointListList>, Point) {
   #[inline]
   fn divide(mut v1: Box<PointListList>, v2: Point) -> Data {
      for src in v1.iter_mut() {
         for point in src.iter_mut() {
            point.x /= v2.x;
            point.y /= v2.y;
         }
      }

      Data::PointListList(v1)
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
      (Data::Int(v1), Data::Int(v2)) => <(i64, i64)>::subtract(v1, v2),

      (Data::Float(v1), Data::Int(v2)) => <(f64, i64)>::subtract(v1, v2),
      (Data::Int(v1), Data::Float(v2)) => <(i64, f64)>::subtract(v1, v2),

      (Data::Point(v1), Data::Int(v2)) => <(Point, i64)>::subtract(v1, v2),

      (Data::Point(v1), Data::Point(v2)) => <(Point, Point)>::subtract(v1, v2),

      (Data::PointList(v1), Data::Point(v2)) => <(Box<PointList>, Point)>::subtract(v1, v2),

      (Data::PointListList(v1), Data::Point(v2)) => <(Box<PointListList>, Point)>::subtract(v1, v2),

      _ => NONE
   }
}

trait SubtractTrait<T1, T2> {
   fn subtract(v1: T1, v2: T2) -> Data;
}

impl SubtractTrait<i64, i64> for (i64, i64) {
   #[inline]
   fn subtract(v1: i64, v2: i64) -> Data {
      Data::Int(v1 - v2)
   }
}

impl SubtractTrait<f64, i64> for (f64, i64) {
   #[inline]
   fn subtract(v1: f64, v2: i64) -> Data {
      Data::Float(v1 - v2 as f64)
   }
}

impl SubtractTrait<i64, f64> for (i64, f64) {
   #[inline]
   fn subtract(v1: i64, v2: f64) -> Data {
      Data::Float(v1 as f64 - v2)
   }
}

impl SubtractTrait<Point, i64> for (Point, i64) {
   #[inline]
   fn subtract(mut v1: Point, v2: i64) -> Data {
      v1.x -= v2;
      v1.y -= v2;

      Data::Point(v1)
   }
}

impl SubtractTrait<Point, Point> for (Point, Point) {
   #[inline]
   fn subtract(mut v1: Point, v2: Point) -> Data {
      v1.x -= v2.x;
      v1.y -= v2.y;

      Data::Point(v1)
   }
}

impl SubtractTrait<Box<PointList>, Point> for (Box<PointList>, Point) {
   #[inline]
   fn subtract(mut v1: Box<PointList>, v2: Point) -> Data {
      for point in v1.iter_mut() {
         point.x -= v2.x;
         point.y -= v2.y;
      }

      Data::PointList(v1)
   }
}

impl SubtractTrait<Box<PointListList>, Point> for (Box<PointListList>, Point) {
   #[inline]
   fn subtract(mut v1: Box<PointListList>, v2: Point) -> Data {
      for src in v1.iter_mut() {
         for point in src.iter_mut() {
            point.x -= v2.x;
            point.y -= v2.y;
         }
      }

      Data::PointListList(v1)
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
      (Data::Int(v1), Data::Int(v2)) => <(i64, i64)>::multiply(v1, v2),

      (Data::Float(v1), Data::Int(v2)) => <(f64, i64)>::multiply(v1, v2),
      (Data::Int(v1), Data::Float(v2)) => <(f64, i64)>::multiply(v2, v1),

      (Data::Point(v1), Data::Int(v2)) => <(Point, i64)>::multiply(v1, v2),
      (Data::Int(v1), Data::Point(v2)) => <(Point, i64)>::multiply(v2, v1),

      (Data::Point(v1), Data::Point(v2)) => <(Point, Point)>::multiply(v1, v2),

      (Data::PointList(v1), Data::Point(v2)) => <(Box<PointList>, Point)>::multiply(v1, v2),
      (Data::Point(v1), Data::PointList(v2)) => <(Box<PointList>, Point)>::multiply(v2, v1),

      (Data::PointListList(v1), Data::Point(v2)) => <(Box<PointListList>, Point)>::multiply(v1, v2),
      (Data::Point(v1), Data::PointListList(v2)) => <(Box<PointListList>, Point)>::multiply(v2, v1),

      _ => NONE
   }
}

trait MultiplyTrait<T1, T2> {
   fn multiply(v1: T1, v2: T2) -> Data;
}

impl MultiplyTrait<i64, i64> for (i64, i64) {
   #[inline]
   fn multiply(v1: i64, v2: i64) -> Data {
      Data::Int(v1 * v2)
   }
}

impl MultiplyTrait<f64, i64> for (f64, i64) {
   #[inline]
   fn multiply(v1: f64, v2: i64) -> Data {
      Data::Float(v1 * v2 as f64)
   }
}

impl MultiplyTrait<Point, i64> for (Point, i64) {
   #[inline]
   fn multiply(mut v1: Point, v2: i64) -> Data {
      v1.x *= v2;
      v1.y *= v2;

      Data::Point(v1)
   }
}

impl MultiplyTrait<Point, Point> for (Point, Point) {
   #[inline]
   fn multiply(mut v1: Point, v2: Point) -> Data {
      v1.x *= v2.x;
      v1.y *= v2.y;

      Data::Point(v1)
   }
}

impl MultiplyTrait<Box<PointList>, Point> for (Box<PointList>, Point) {
   #[inline]
   fn multiply(mut v1: Box<PointList>, v2: Point) -> Data {
      for point in v1.iter_mut() {
         point.x *= v2.x;
         point.y *= v2.y;
      }

      Data::PointList(v1)
   }
}

impl MultiplyTrait<Box<PointListList>, Point> for (Box<PointListList>, Point) {
   #[inline]
   fn multiply(mut v1: Box<PointListList>, v2: Point) -> Data {
      for src in v1.iter_mut() {
         for point in src.iter_mut() {
            point.x *= v2.x;
            point.y *= v2.y;
         }
      }

      Data::PointListList(v1)
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

trait NthTrait<T1> {
   fn nth(object: T1, index: i64) -> Data;
}

impl NthTrait<Point> for Point {
   #[inline]
   fn nth(object: Point, index: i64) -> Data {
      match index {
         0 => Data::Int(object.x),
         1 => Data::Int(object.y),
         _ => NONE,
      }
   }
}

impl NthTrait<RGB> for RGB {
   #[inline]
   fn nth(object: RGB, index: i64) -> Data {
      match index {
         0 => Data::Int(object.r as i64),
         1 => Data::Int(object.g as i64),
         2 => Data::Int(object.b as i64),
         _ => NONE,
      }
   }
}


impl NthTrait<Box<BBox>> for Box<BBox> {
   #[inline]
   fn nth(object: Box<BBox>, index: i64) -> Data {
      match index {
         0 => Data::Point(object.p1),
         1 => Data::Point(object.p2),
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
         <(Point, f64)>::rotate(target, origin, angle),

      (Data::Point(target), Data::Point(origin), Data::Int(angle)) =>
         <(Point, i64)>::rotate(target, origin, angle),

      (Data::PointList(target), Data::Point(origin), Data::Float(angle)) =>
         <(Box<PointList>, f64)>::rotate(target, origin, angle),

      (Data::PointList(target), Data::Point(origin), Data::Int(angle)) =>
         <(Box<PointList>, i64)>::rotate(target, origin, angle),

      (Data::PointListList(target), Data::Point(origin), Data::Float(angle)) =>
         <(Box<PointListList>, f64)>::rotate(target, origin, angle),

      (Data::PointListList(target), Data::Point(origin), Data::Int(angle)) =>
         <(Box<PointListList>, i64)>::rotate(target, origin, angle),

      _ => NONE
   }
}

trait RotateTrait<T1, T2> {
   fn rotate(target: T1, origin: Point, angle: T2) -> Data;
}

impl RotateTrait<Point, f64> for (Point, f64) {
   #[inline]
   fn rotate(mut target: Point, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      let mut x = target.x as f64;
      let mut y = target.y as f64;

      x -= cx;
      y -= cy;

      target.x = (x * c - y * s + cx) as i64;
      target.y = (x * s + y * c + cy) as i64;

      Data::Point(target)
   }
}

impl RotateTrait<Point, i64> for (Point, i64) {
   #[inline]
   fn rotate(target: Point, origin: Point, angle: i64) -> Data {
      <(Point, f64)>::rotate(target, origin, angle as f64)
   }
}

impl RotateTrait<Box<PointList>, f64> for (Box<PointList>, f64) {
   #[inline]
   fn rotate(mut target: Box<PointList>, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for tuple in target.iter_mut() {
         let mut x = tuple.x as f64;
         let mut y = tuple.y as f64;

         x -= cx;
         y -= cy;

         tuple.x = (x * c - y * s + cx) as i64;
         tuple.y = (x * s + y * c + cy) as i64;
      }

      Data::PointList(target)
   }
}

impl RotateTrait<Box<PointList>, i64> for (Box<PointList>, i64) {
   #[inline]
   fn rotate(target: Box<PointList>, origin: Point, angle: i64) -> Data {
      <(Box<PointList>, f64)>::rotate(target, origin, angle as f64)
   }
}

impl RotateTrait<Box<PointListList>, f64> for (Box<PointListList>, f64) {
   #[inline]
   fn rotate(mut target: Box<PointListList>, origin: Point, angle: f64) -> Data {
      let cx = origin.x as f64;
      let cy = origin.y as f64;

      let radians = angle.to_radians();

      let s = radians.sin();
      let c = radians.cos();

      for outer in target.iter_mut() {
         for tuple in outer.iter_mut() {
            let mut x = tuple.x as f64;
            let mut y = tuple.y as f64;

            x -= cx;
            y -= cy;

            tuple.x = (x * c - y * s + cx) as i64;
            tuple.y = (x * s + y * c + cy) as i64;
         }
      }

      Data::PointListList(target)
   }
}

impl RotateTrait<Box<PointListList>, i64> for (Box<PointListList>, i64) {
   #[inline]
   fn rotate(target: Box<PointListList>, origin: Point, angle: i64) -> Data {
      <(Box<PointListList>, f64)>::rotate(target, origin, angle as f64)
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
      Data::Point(object) => <Point>::bbox(object),

      Data::PointList(object) => <Box<PointList>>::bbox(object),

      Data::PointListList(object) => <Box<PointListList>>::bbox(object),

      _ => NONE
   }
}

trait BBoxTrait<T1> {
   fn bbox(object: T1) -> Data;
}

impl BBoxTrait<Point> for Point {
   #[inline]
   fn bbox(object: Point) -> Data {
      Data::BBox(
         Box::new(
            BBox::new(object, object)
         )
      )
   }
}

impl BBoxTrait<Box<PointList>> for Box<PointList> {
   #[inline]
   fn bbox(object: Box<PointList>) -> Data {
      if object.len() == 0 {
         return NONE;
      }

      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for tuple in object.iter() {
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

impl BBoxTrait<Box<PointListList>> for Box<PointListList> {
   #[inline]
   fn bbox(object: Box<PointListList>) -> Data {
      let mut top = i64::MAX;
      let mut bottom = i64::MIN;

      let mut left = i64::MAX;
      let mut right = i64::MIN;

      for outer in object.iter() {
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
      Data::Point(object) => <Point>::center(object),

      Data::BBox(object) => <BBox>::center(*object),

      Data::PointList(object) => <Box<PointList>>::center(object),

      Data::PointListList(object) => <Box<PointListList>>::center(object),

      _ => NONE
   }
}

trait CenterTrait<T1> {
   fn center(object: T1) -> Data;
}

impl CenterTrait<Point> for Point {
   #[inline]
   fn center(object: Point) -> Data {
      Data::Point(object)
   }
}

impl CenterTrait<BBox> for BBox {
   #[inline]
   fn center(object: BBox) -> Data {
      let x = (object.p1.x + object.p2.x) / 2;
      let y = (object.p1.y + object.p1.y) / 2;
      Data::Point(Point::new(x, y))
   }
}

impl CenterTrait<Box<PointList>> for Box<PointList> {
   #[inline]
   fn center(object: Box<PointList>) -> Data {
      let bbox = <Box<PointList>>::bbox(object);

      match bbox {
         Data::BBox(bbox) => <BBox>::center(*bbox),
         _ => NONE,
      }
   }
}

impl CenterTrait<Box<PointListList>> for Box<PointListList> {
   #[inline]
   fn center(object: Box<PointListList>) -> Data {
      let bbox = <Box<PointListList>>::bbox(object);

      match bbox {
         Data::BBox(bbox) => <BBox>::center(*bbox),
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

impl BuildList {
   #[inline]
   fn create_int_list(&self, node: &Node, state: &mut [Vec<Data>], first: i64) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Int(value) = input {
            result.push(value);
         }
      }

      Data::IntList(Box::new(result))
   }

   #[inline]
   fn create_float_list(&self, node: &Node, state: &mut [Vec<Data>], first: f64) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Float(value) = input {
            result.push(value);
         }
      }

      Data::FloatList(Box::new(result))
   }

   #[inline]
   fn create_bool_list(&self, node: &Node, state: &mut [Vec<Data>], first: bool) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Bool(value) = input {
            result.push(value);
         }
      }

      Data::BoolList(Box::new(result))
   }

   #[inline]
   fn create_point_list(&self, node: &Node, state: &mut [Vec<Data>], first: Point) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Point(value) = input {
            result.push(value);
         }
      }

      Data::PointList(Box::new(result))
   }

   #[inline]
   fn create_rgb_list(&self, node: &Node, state: &mut [Vec<Data>], first: RGB) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Rgb(value) = input {
            result.push(value);
         }
      }

      Data::RgbList(Box::new(result))
   }

   #[inline]
   fn create_poly_list(&self, node: &Node, state: &mut [Vec<Data>], first: Box<Poly>) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(*first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Poly(value) = input {
            result.push(*value);
         }
      }

      Data::PolyList(Box::new(result))
   }

   #[inline]
   fn create_layer_list(&self, node: &Node, state: &mut [Vec<Data>], first: Box<Layer>) -> Data {
      let mut result = Vec::with_capacity(node.len());

      result.push(*first);

      for i in 1..node.len() {
         let input = node.input(state, i);

         if let Data::Layer(value) = input {
            result.push(*value);
         }
      }

      Data::LayerList(Box::new(result))
   }
}

impl Operator for BuildList {
   #[inline]
   fn new() -> Self {
      BuildList { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Option<Data> {
      let first = node.input(state, 0);

      let result = match first {
         Data::Int(first) => self.create_int_list(node, state, first),
         Data::Float(first) => self.create_float_list(node, state, first),
         Data::Bool(first) => self.create_bool_list(node, state, first),
         Data::Point(first) => self.create_point_list(node, state, first),
         Data::Rgb(first) => self.create_rgb_list(node, state, first),
         Data::Poly(first) => self.create_poly_list(node, state, first),
         Data::Layer(first) => self.create_layer_list(node, state, first),
         _ => NONE
      };

      Some(result)
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
         (Data::PointList(v1), Data::Rgb(v2)) => <(Box<PointList>, RGB)>::build_poly(v1, v2),
         _ => NONE
      };

      Some(result)
   }
}

trait BuildPolyTrait<T1, T2> {
   fn build_poly(v1: T1, v2: T2) -> Data;
}

impl BuildPolyTrait<Box<PointList>, RGB> for (Box<PointList>, RGB) {
   #[inline]
   fn build_poly(array: Box<PointList>, color: RGB) -> Data {
      let color = RGB::new(color.r, color.g, color.b);

      let poly = Poly::new(*array, color);

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


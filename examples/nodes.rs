extern crate polydraw;

use polydraw::Application;
use polydraw::devel::{Scene, Poly, DevelRenderer};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


type TPoint = (i64, i64);
type TPoly = Vec<Vec<TPoint>>;

#[derive(Debug)]
#[allow(dead_code)]
enum Data {
   None,
   I64(i64),
   F64(f64),
   Point(TPoint),
   Poly(TPoly),
}


trait Node {
   fn new(init1: Data, init2: Data, init3: Data, init4: Data) -> Self;

   fn process(&self, arg1: &Data, arg2: &Data, arg3: &Data, arg4: &Data) -> Data;
}

struct AddNode {
   init1: Data,
   init2: Data,
}

impl Node for AddNode {
   #[inline]
   fn new(init1: Data, init2: Data, _: Data, _: Data) -> Self {
      AddNode {
         init1: init1,
         init2: init2,
      }
   }

   #[inline]
   fn process(&self, arg1: &Data, arg2: &Data, _: &Data, _: &Data) -> Data {
      let in1 = actual(arg1, &self.init1);
      let in2 = actual(arg2, &self.init2);

      match (in1, in2) {
         (&Data::I64(ref v1), &Data::I64(ref v2)) => <(i64, i64)>::add(v1, v2),

         (&Data::F64(ref v1), &Data::I64(ref v2)) => <(f64, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::F64(ref v2)) => <(f64, i64)>::add(v2, v1),

         (&Data::Point(ref v1), &Data::I64(ref v2)) => <(TPoint, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::Point(ref v2)) => <(TPoint, i64)>::add(v2, v1),

         (&Data::Poly(ref v1), &Data::Point(ref v2)) => <(TPoly, TPoint)>::add(v1, v2),
         (&Data::Point(ref v1), &Data::Poly(ref v2)) => <(TPoly, TPoint)>::add(v2, v1),

         _ => Data::None
      }
   }
}

trait Add<T1, T2> {
   fn add(v1: &T1, v2: &T2) -> Data;
}

impl Add<i64, i64> for (i64, i64) {
   #[inline]
   fn add(v1: &i64, v2: &i64) -> Data {
      Data::I64(*v1 + *v2)
   }
}

impl Add<f64, i64> for (f64, i64) {
   #[inline]
   fn add(v1: &f64, v2: &i64) -> Data {
      Data::F64(*v1 + *v2 as f64)
   }
}

impl Add<TPoint, i64> for (TPoint, i64) {
   #[inline]
   fn add(v1: &TPoint, v2: &i64) -> Data {
      Data::Point((v1.0 + *v2, v1.1 + *v2))
   }
}

impl Add<TPoly, TPoint> for (TPoly, TPoint) {
   #[inline]
   fn add(v1: &TPoly, v2: &TPoint) -> Data {
      let mut outer = Vec::with_capacity(v1.len());

      for src in v1 {
         let mut inner = Vec::with_capacity(src.len());

         for tuple in src {
            inner.push((tuple.0 + v2.0, tuple.1 + v2.1));
         }

         outer.push(inner);
      }

      Data::Poly(outer)
   }
}

#[inline]
fn actual<'a>(passed: &'a Data, initial: &'a Data) -> &'a Data {
   match passed {
      &Data::None => initial,
      _ => passed
   }
}

#[inline]
fn _poly_from_data(data: &TPoly) -> Poly {
   let outer = _points_from_coords(&data[0]);

   let mut inner = Vec::new();

   for inner_data in &data[1..] {
      inner.push(
         _points_from_coords(inner_data)
      );
   }

   let poly = Poly::new_with_holes(
      outer, inner, RGB::new(81, 180, 200),
   );

   poly
}

#[inline]
fn _points_from_coords(coords: &[(i64, i64)]) -> Vec<Point> {
   let mut points = Vec::new();

   for &(x, y) in coords.iter() {
      points.push(Point::new(x + 120, y + 120))
   }

   points
}

fn main() {
   let mut scene = Scene::new();

   let source = vec![vec![
      (90, 1200),
      (261, 1735),
      (1443, 410),
      (493, 174),
   ]];

   let add = AddNode::new(Data::None, Data::None, Data::None, Data::None);

   let destination = add.process(
      &Data::Poly(source), &Data::Point((957, 223)), &Data::None, &Data::None
   );

   match destination {
      Data::Poly(data) => scene.push(_poly_from_data(&data)),
      _ => {}
   }

   let mut renderer = DevelRenderer::new(scene);

   Application::new()
      .renderer(&mut renderer)
      .title("Nodes")
      .size(1200, 800)
      .run();
}


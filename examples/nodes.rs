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

struct Add {
   init1: Data,
   init2: Data,
}

impl Add {
}

impl Node for Add {
   #[inline]
   fn new(init1: Data, init2: Data, _: Data, _: Data) -> Self {
      Add {
         init1: init1,
         init2: init2,
      }
   }

   #[inline]
   fn process(&self, arg1: &Data, arg2: &Data, _: &Data, _: &Data) -> Data {
      let val1 = actual(arg1, &self.init1);
      let val2 = actual(arg2, &self.init2);

      match val1 {
         &Data::I64(ref left) => match val2 {
            &Data::I64(ref right) => add_i64_i64(left, right),
            &Data::F64(ref right) => Data::F64(*left as f64 + *right),
            &Data::Point(ref right) => add_point_i64(right, left),
            _ => Data::None
         },
         &Data::F64(ref left) => match val2 {
            &Data::I64(ref right) => Data::F64(*left + *right as f64),
            &Data::F64(ref right) => Data::F64(*left + *right),
            _ => Data::None
         },
         &Data::Poly(ref left) => match val2 {
            &Data::Point(ref right) => add_poly_point(left, right),
            _ => Data::None
         },
         &Data::Point(ref left) => match val2 {
            &Data::Poly(ref right) => add_poly_point(right, left),
            _ => Data::None
         },
         _ => Data::None
      }
   }
}

#[inline]
fn add_i64_i64(left: &i64, right: &i64) -> Data {
   Data::I64(*left + *right)
}

#[inline]
fn add_point_i64(left: &TPoint, right: &i64) -> Data {
   Data::Point((left.0 + *right, left.1 + *right))
}

#[inline]
fn add_poly_point(left: &TPoly, right: &TPoint) -> Data {
   let mut group = Vec::with_capacity(left.len());

   let (add_x, add_y) = *right;

   for src in left {
      let mut contour = Vec::with_capacity(src.len());

      for point in src {
         let (x, y) = *point;
         contour.push((x + add_x, y + add_y));
      }

      group.push(contour);
   }

   Data::Poly(group)
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

   let add = Add::new(Data::None, Data::None, Data::None, Data::None);

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


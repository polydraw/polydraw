extern crate polydraw;
extern crate toml;

use polydraw::{Renderer, Application, Frame};
use polydraw::devel::{Scene, Poly, DevelRenderer};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


const NODE_DEFS: &'static str = r#"

   [frame]
   type = "frame"

   [poly-points]
   type = "[(i64, i64)]"
   value = [ [0, 0], [90, 1200], [261, 1735], [1443, 410] ]

   [translate-point]
   type = "pair"
   in-1 = "frame"
   in-2 = { type = "i64", value = 0 }

   [add-operator]
   type = "add"
   in-1 = "poly-points"
   in-2 = "translate-point"

   [poly]
   type = "poly"
   in-1 = "add-operator"
   in-2 = { type = "(u8, u8, u8)", value = [0, 127, 255] }

   [layer]
   type = "layer"
   in-1 = "poly"

   [doc]
   type = "doc"
   in-1 = "layer"

"#;


type I64I64 = (i64, i64);
type U8U8U8 = (u8, u8, u8);
type VVI64I64 = Vec<Vec<I64I64>>;

#[derive(Debug)]
#[allow(dead_code)]
enum Data {
   None,
   I64(i64),
   F64(f64),
   I64I64(I64I64),
   U8U8U8(U8U8U8),
   VVI64I64(VVI64I64),
}

const NONE: Data = Data::None;

trait Node {
   fn process(&self, args: &[&Data]) -> Data;
}

struct AddNode {
   first: Data,
   second: Data,
}

impl AddNode {
   #[inline]
   fn new(first: Data, second: Data) -> Self {
      AddNode {
         first: first,
         second: second,
      }
   }
}

impl Node for AddNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      let in1 = in_value(args, 0, &self.first);
      let in2 = in_value(args, 1, &self.second);

      match (in1, in2) {
         (&Data::I64(ref v1), &Data::I64(ref v2)) => <(i64, i64)>::add(v1, v2),

         (&Data::F64(ref v1), &Data::I64(ref v2)) => <(f64, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::F64(ref v2)) => <(f64, i64)>::add(v2, v1),

         (&Data::I64I64(ref v1), &Data::I64(ref v2)) => <(I64I64, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::I64I64(ref v2)) => <(I64I64, i64)>::add(v2, v1),

         (&Data::VVI64I64(ref v1), &Data::I64I64(ref v2)) => <(VVI64I64, I64I64)>::add(v1, v2),
         (&Data::I64I64(ref v1), &Data::VVI64I64(ref v2)) => <(VVI64I64, I64I64)>::add(v2, v1),

         _ => NONE
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

impl Add<I64I64, i64> for (I64I64, i64) {
   #[inline]
   fn add(v1: &I64I64, v2: &i64) -> Data {
      Data::I64I64((v1.0 + *v2, v1.1 + *v2))
   }
}

impl Add<VVI64I64, I64I64> for (VVI64I64, I64I64) {
   #[inline]
   fn add(v1: &VVI64I64, v2: &I64I64) -> Data {
      let mut outer = Vec::with_capacity(v1.len());

      for src in v1 {
         let mut inner = Vec::with_capacity(src.len());

         for tuple in src {
            inner.push((tuple.0 + v2.0, tuple.1 + v2.1));
         }

         outer.push(inner);
      }

      Data::VVI64I64(outer)
   }
}

#[inline]
fn in_value<'a>(args: &'a[&'a Data], index: usize, initial: &'a Data) -> &'a Data {
   match args.get(index) {
      Some(passed) => match *passed {
         &Data::None => initial,
         _ => *passed
      },
      None => initial
   }
}

#[inline]
fn poly_from_data(data: &VVI64I64) -> Poly {
   let outer = points_from_coords(&data[0]);

   let mut inner = Vec::new();

   for inner_data in &data[1..] {
      inner.push(
         points_from_coords(inner_data)
      );
   }

   let poly = Poly::new_with_holes(
      outer, inner, RGB::new(81, 180, 200),
   );

   poly
}

#[inline]
fn points_from_coords(coords: &[(i64, i64)]) -> Vec<Point> {
   let mut points = Vec::new();

   for &(x, y) in coords.iter() {
      points.push(Point::new(x + 120, y + 120))
   }

   points
}

struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
}

impl NodeRenderer {
   #[inline]
   pub fn new() -> Self {
      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
      }
   }
}

impl Renderer for NodeRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      let mut parser = NodeParser::new();
      parser.parse(NODE_DEFS);

      self.renderer.init(width, height);
   }

   #[inline]
   fn render(&mut self, frame: &mut Frame) {
      let mut scene = Scene::new();

      let source = vec![vec![
         (90, 1200),
         (261, 1735),
         (1443, 410),
         (493, 174),
      ]];

      let add = AddNode::new(NONE, NONE);

      let destination = add.process(
         &[&Data::VVI64I64(source), &Data::I64I64((self.frame, 0))]
      );

      match destination {
         Data::VVI64I64(data) => scene.push(poly_from_data(&data)),
         _ => {}
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);

      self.frame += 1;
   }
}

struct NodeParser;

impl NodeParser {
   fn new() -> Self {
      NodeParser {}
   }

   fn parse(&mut self, node_defs: &str) {
      let mut parser = toml::Parser::new(node_defs);

      match parser.parse() {
         Some(everything) => {
            for (node_id, value) in everything.iter() {
               match value {
                  &toml::Value::Table(ref node_table) => {
                     self.process_node(node_id, node_table);
                  },
                  _ => {
                     println!("`{}` is not a table ", node_id);
                  }
               }
            }
         },
         None => {
            println!("parse errors: {:?}", parser.errors);
         }
      }
   }

   fn process_node(&mut self, node_id: &str, node_table: &toml::Table) {
      println!("{}: {:?}", node_id, node_table);
   }
}

fn main() {
   let mut renderer = NodeRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Nodes")
      .size(1200, 800)
      .run();
}


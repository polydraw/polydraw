extern crate polydraw;
extern crate toml;

use std::fmt;
use std::collections::HashMap;

use polydraw::{Renderer, Application, Frame};
use polydraw::devel::{Scene, Poly, DevelRenderer};
use polydraw::geom::point::Point;
use polydraw::draw::RGB;


const NODE_DEFS: &'static str = r#"

   [poly-points]
   type = "[(i64, i64)]"
   data = [ [0, 0], [90, 1200], [261, 1735], [1443, 410] ]

   [translate-point]
   type = "join"
   data = [
      { from = "frame" },
      { type = "i64", data = 0 } ]

   [add-operator]
   type = "add"
   data = [
      { from = "poly-points" },
      { from = "translate-point" } ]

   [poly]
   type = "poly"
   data = [
      { from = "add-operator" },
      { type = "(u8, u8, u8)", data = [0, 127, 255] } ]

   [poly-list]
   type = "list"
   data = [
      { from = "poly" } ]

   [layer]
   type = "layer"
   data = [
      { from = "poly-list" } ]

   [doc]
   type = "doc"
   data = [
      { from = "layer" } ]

"#;

#[derive(Debug)]
struct Layer;

#[derive(Debug)]
struct Document;

type U8U8U8 = (u8, u8, u8);

type I64I64 = (i64, i64);
type VI64I64 = Vec<I64I64>;
type VVI64I64 = Vec<Vec<I64I64>>;

type PolyBox = Box<Poly>;
type VPolyBox = Vec<Box<Poly>>;

type LayerBox = Box<Layer>;
type DocBox = Box<Document>;

#[derive(Debug)]
#[allow(dead_code)]
enum Data {
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
   Doc(DocBox),
}

const NONE: Data = Data::None;

trait Node where Self: fmt::Debug {
   fn process(&self, args: &[&Data]) -> Data;
}

trait ProcessNode: Node {
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> where Self: Sized;
}

trait DataTypeNode: Node {
   fn new_boxed(data: Data) -> Box<Self> where Self: Sized;
}

#[derive(Debug)]
struct AddNode {
   defaults: Vec<Data>,
}

impl ProcessNode for AddNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         AddNode {
            defaults: defaults,
         }
      )
   }
}

impl Node for AddNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      let in1 = in_value(args, 0, &self.defaults[0]);
      let in2 = in_value(args, 1, &self.defaults[1]);

      match (in1, in2) {
         (&Data::I64(ref v1), &Data::I64(ref v2)) => <(i64, i64)>::add(v1, v2),

         (&Data::F64(ref v1), &Data::I64(ref v2)) => <(f64, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::F64(ref v2)) => <(f64, i64)>::add(v2, v1),

         (&Data::I64I64(ref v1), &Data::I64(ref v2)) => <(I64I64, i64)>::add(v1, v2),
         (&Data::I64(ref v1), &Data::I64I64(ref v2)) => <(I64I64, i64)>::add(v2, v1),

         (&Data::VI64I64(ref v1), &Data::I64I64(ref v2)) => <(VI64I64, I64I64)>::add(v1, v2),
         (&Data::I64I64(ref v1), &Data::VI64I64(ref v2)) => <(VI64I64, I64I64)>::add(v2, v1),

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

impl Add<VI64I64, I64I64> for (VI64I64, I64I64) {
   #[inline]
   fn add(v1: &VI64I64, v2: &I64I64) -> Data {
      let mut result = Vec::with_capacity(v1.len());

      for tuple in v1 {
         result.push((tuple.0 + v2.0, tuple.1 + v2.1));
      }

      Data::VI64I64(result)
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


#[derive(Debug)]
struct JoinNode {
   defaults: Vec<Data>,
}

impl JoinNode {
   #[inline]
   fn process_two(&self, args: &[&Data]) -> Data {
      let in1 = in_value(args, 0, &self.defaults[0]);
      let in2 = in_value(args, 1, &self.defaults[1]);

      match (in1, in2) {
         (&Data::I64(v1), &Data::I64(v2)) => Data::I64I64((v1, v2)),

         _ => NONE
      }
   }

   #[inline]
   fn process_three(&self, args: &[&Data]) -> Data {
      let in1 = in_value(args, 0, &self.defaults[0]);
      let in2 = in_value(args, 1, &self.defaults[1]);
      let in3 = in_value(args, 2, &self.defaults[2]);

      match (in1, in2, in3) {
         (&Data::U8(v1), &Data::U8(v2), &Data::U8(v3)) => Data::U8U8U8((v1, v2, v3)),

         _ => NONE
      }
   }
}

impl ProcessNode for JoinNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         JoinNode {
            defaults: defaults,
         }
      )
   }
}

impl Node for JoinNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      match self.defaults.len() {
         2 => self.process_two(args),
         3 => self.process_three(args),
         _ => Data::None
      }
   }
}


#[derive(Debug)]
struct ListNode {
   defaults: Vec<Data>,
}

impl ProcessNode for ListNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         ListNode {
            defaults: defaults,
         }
      )
   }
}

impl ListNode {
   #[inline]
   fn create_poly_list(&self, args: &[&Data]) -> Data {
      let mut result = Vec::with_capacity(args.len());

      for i in 0..args.len() {
         let element = in_value(args, i, &self.defaults[i]);

         match element {
            &Data::Poly(ref poly) => result.push((*poly).clone()),
            _ => {}
         }
      }

      Data::VPoly(result)
   }
}

impl Node for ListNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      let first = in_value(args, 0, &self.defaults[0]);

      match first {
         &Data::Poly(_) => self.create_poly_list(args),
         _ => NONE
      }
   }
}


#[derive(Debug)]
struct PolyNode {
   defaults: Vec<Data>,
}

impl ProcessNode for PolyNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         PolyNode {
            defaults: defaults,
         }
      )
   }
}

impl Node for PolyNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      let points = in_value(args, 0, &self.defaults[0]);
      let color = in_value(args, 1, &self.defaults[1]);

      match (points, color) {
         (&Data::VI64I64(ref v1), &Data::U8U8U8(ref v2)) => <(VI64I64, U8U8U8)>::create_poly(v1, v2),

         _ => NONE
      }
   }
}

trait PolyMake<T1, T2> {
   fn create_poly(v1: &T1, v2: &T2) -> Data;
}

impl PolyMake<VI64I64, U8U8U8> for (VI64I64, U8U8U8) {
   #[inline]
   fn create_poly(array: &VI64I64, color: &U8U8U8) -> Data {
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
struct LayerNode {
   defaults: Vec<Data>,
}

impl ProcessNode for LayerNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         LayerNode {
            defaults: defaults,
         }
      )
   }
}

impl Node for LayerNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      let polys_data = in_value(args, 0, &self.defaults[0]);

      match polys_data {
         &Data::VPoly(_) => {
            Data::Layer(Box::new(Layer {}))
         },
         _ => NONE
      }
   }
}


#[derive(Debug)]
struct DocNode {
   list_node: ListNode,
}

impl ProcessNode for DocNode {
   #[inline]
   fn new_boxed(defaults: Vec<Data>) -> Box<Self> {
      Box::new(
         DocNode {
            list_node: ListNode {
               defaults: defaults,
            }
         }
      )
   }
}

impl Node for DocNode {
   #[inline]
   fn process(&self, args: &[&Data]) -> Data {
      self.list_node.process(args)
   }
}


#[derive(Debug)]
struct DataNode {
   data: Data,
}

impl DataTypeNode for DataNode {
   #[inline]
   fn new_boxed(data: Data) -> Box<Self> {
      Box::new(
         DataNode {
            data: data,
         }
      )
   }
}

impl Node for DataNode {
   #[inline]
   fn process(&self, _: &[&Data]) -> Data {
      NONE
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
      parse(NODE_DEFS);

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

      let add = AddNode::new_boxed(vec![NONE, NONE]);

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

fn parse(node_defs: &str) {
   let mut parser = toml::Parser::new(node_defs);

   match parser.parse() {
      Some(all_tables) => {
         let mut index_map = HashMap::new();
         index_map.insert("frame", 0);

         for (index, node_id) in all_tables.keys().enumerate() {
            println!("NODE {} {}", index, node_id);
            index_map.insert(node_id.as_str(), index + 1);
         }

         for (node_id, value) in all_tables.iter() {
            match value {
               &toml::Value::Table(ref node_table) => {
                  process_node(node_id, node_table, &index_map);
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

fn process_node(node_id: &str, node_table: &toml::Table, index_map: &HashMap<&str, usize>) {
   let node_type = extract_node_type(node_id, node_table);

   println!("TYPE: {}", node_type);

   println!("{:?}", node_table);

   let node = match node_type.as_ref() {
      "add" => create_processing_node::<AddNode>(node_id, node_table, index_map),
      "join" => create_processing_node::<JoinNode>(node_id, node_table, index_map),
      "list" => create_processing_node::<ListNode>(node_id, node_table, index_map),

      "poly" => create_processing_node::<PolyNode>(node_id, node_table, index_map),
      "layer" => create_processing_node::<LayerNode>(node_id, node_table, index_map),
      "doc" => create_processing_node::<DocNode>(node_id, node_table, index_map),

      "[(i64, i64)]" => create_data_node::<DataNode>(node_id, node_table),

      _ => {
         println!("Unknown node type {:?} for: {}", node_type, node_id);
         println!("");
         return;
      }
   };

   println!("NODE {:?}", node);

   println!("");
}


fn create_processing_node<T: 'static + ProcessNode>(
   node_id: &str, node_table: &toml::Table, index_map: &HashMap<&str, usize>
) -> Box<Node> {

   let data_value = extract_data_value(node_id, node_table);

   println!("DATA {:?}", data_value);

   let (defaults, inlets) = to_defaults(node_id, data_value, index_map);

   println!("defaults: {:?}", defaults);
   println!("inlets: {:?}", inlets);

   T::new_boxed(defaults)
}


fn create_data_node<T: 'static + DataTypeNode>(node_id: &str, node_table: &toml::Table) -> Box<Node> {
   let data = extract_table_data(node_id, node_table);

   T::new_boxed(data)
}


fn extract_node_type<'a>(node_id: &str, node_table: &'a toml::Table) -> &'a str {
   match node_table.get("type") {
      Some(type_value) => {
         match type_value {
            &toml::Value::String(ref node_type) => node_type,
            _ => {
               panic!("node type not a string: {}", node_id);
            }
         }
      },
      None => {
         panic!("node without type: {}", node_id);
      }
   }
}


fn extract_data_value<'a>(node_id: &str, node_table: &'a toml::Table) -> &'a toml::Value {
   match node_table.get("data") {
      Some(data_value) => {
         data_value
      },
      None => {
         panic!("node without data: {}", node_id);
      }
   }
}


fn to_defaults(
   node_id: &str,
   data: &toml::Value,
   index_map: &HashMap<&str, usize>
) -> (Vec<Data>, Vec<Option<usize>>) {

   let array = match data {
      &toml::Value::Array(ref array) => array,
      _ => {
         panic!("data is not an array: {}", node_id);
      }
   };

   let mut defaults = Vec::with_capacity(array.len());
   let mut inlets = Vec::with_capacity(array.len());

   for item in array.iter() {
      let table = match item {
         &toml::Value::Table(ref table) => table,
         _ => {
            panic!("value is not a table {:?}: {}", item, node_id);
         }
      };

      println!("item: {:?}", table);

      match table.get("from") {
         Some(from) => {

            let in_id = match from {
               &toml::Value::String(ref in_id) => in_id,
               _ => {
                  panic!("From is not a string {:?}: {}", from, node_id);
               }
            };

            let index = match index_map.get::<str>(in_id) {
               Some(index) => index,
               _ => {
                  panic!("Unrecognized ID {:?}: {}", in_id, node_id);
               }
            };

            println!("IN ID {:?}", index);

            inlets.push(Some(*index));

            defaults.push(Data::None);

         },
         None => {
            defaults.push(
               extract_table_data(node_id, table)
            );

            inlets.push(None);
         }
      }
   }

   (defaults, inlets)
}


fn extract_table_data(node_id: &str, table: &toml::Table) -> Data {
   let type_str = match table.get("type") {
      Some(type_data) => {
         match type_data {
            &toml::Value::String(ref type_str) => type_str,
            _ => {
               panic!("Type not a string {:?}: {}", type_data, node_id);
            }
         }
      },
      None => {
         panic!("value table without a type {:?}: {}", table, node_id);
      }
   };

   let data = match table.get("data") {
      Some(data) => data,
      None => {
         panic!("value table without a data {:?}: {}", table, node_id);
      }
   };

   match type_str.as_ref() {
      "i64" => toml_to_i64(node_id, data),
      "(u8, u8, u8)" => toml_to_u8u8u8(node_id, data),
      "[(i64, i64)]" => toml_to_vi64i64(node_id, data),
      _ => {
         panic!("Unknown data type {}: {}", type_str, node_id);
      }
   }
}


fn toml_to_i64(node_id: &str, data: &toml::Value) -> Data {
   Data::I64(extract_i64(node_id, data))
}


fn toml_to_u8u8u8(node_id: &str, data: &toml::Value) -> Data {
   match data {
      &toml::Value::Array(ref array) => {
         if array.len() != 3 {
            panic!("Not a triple {:?}: {}", array, node_id);
         }

         let first = extract_u8(node_id, &array[0]);
         let second = extract_u8(node_id, &array[1]);
         let third = extract_u8(node_id, &array[2]);

         Data::U8U8U8((first, second, third))
      },
      _ => {
         panic!("Value not an array {:?}: {}", data, node_id);
      }
   }
}


fn toml_to_vi64i64(node_id: &str, data: &toml::Value) -> Data {
   match data {
      &toml::Value::Array(ref array) => {
         let mut container = Vec::with_capacity(array.len());

         for inner_array in array {
            match inner_array {
               &toml::Value::Array(ref pair) => {
                  if pair.len() != 2 {
                     panic!("Not a pair {:?}: {}", pair, node_id);
                  }

                  let left = extract_i64(node_id, &pair[0]);
                  let right = extract_i64(node_id, &pair[1]);

                  container.push((left, right));
               },
               _ => {
                  panic!("Value not an array {:?}: {}", inner_array, node_id);
               }
            }
         }

         Data::VI64I64(container)
      },
      _ => {
         panic!("Value not an array {:?}: {}", data, node_id);
      }
   }
}


fn extract_i64(node_id: &str, data: &toml::Value) -> i64 {
   match data {
      &toml::Value::Integer(value) => value,
      _ => {
         panic!("Not an integer {:?}: {}", data, node_id);
      }
   }
}


fn extract_u8(node_id: &str, data: &toml::Value) -> u8 {
   extract_i64(node_id, data) as u8
}

fn main() {
   let mut renderer = NodeRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Nodes")
      .size(1200, 800)
      .run();
}


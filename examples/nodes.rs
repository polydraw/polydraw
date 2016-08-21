extern crate polydraw;
extern crate toml;

use std::fmt;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

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

   [artboard]
   type = "artboard"
   data = [
      { from = "layer" } ]

"#;

const NODE_INDEX_OFFSET: usize = 2;

#[derive(Debug)]
struct Layer;

type U8U8U8 = (u8, u8, u8);

type I64I64 = (i64, i64);
type VI64I64 = Vec<I64I64>;
type VVI64I64 = Vec<Vec<I64I64>>;

type PolyBox = Box<Poly>;
type VPolyBox = Vec<Box<Poly>>;

type LayerBox = Box<Layer>;

#[derive(Debug)]
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
}

const NONE: Data = Data::None;

#[derive(Debug)]
struct Node {
   pub operator: Box<Operator>,
   pub consts: Vec<Data>,
   pub inlets: Vec<Option<usize>>,
   pub index: usize,
}

impl Node {
   #[inline]
   fn new(
      operator: Box<Operator>,
      consts: Vec<Data>,
      inlets: Vec<Option<usize>>,
      index: usize,
   ) -> Self {

      Node {
         operator: operator,
         consts: consts,
         inlets: inlets,
         index: index,
      }
   }

   #[inline]
   fn input<'a>(&'a self, data: &'a[Data], index: usize) -> &'a Data {
      match self.inlets.get(index) {
         Some(option) => match *option {
            Some(data_index) => return &data[data_index],
            None => {}
         },
         None => {}
      }

      match self.consts.get(index) {
         Some(ref value) => value,
         None => &data[0]
      }
   }

   #[inline]
   fn len(&self) -> usize {
      assert!(self.consts.len() == self.inlets.len());

      self.consts.len()
   }

   #[inline]
   fn process(&self, data: &[Data]) -> Data {
      self.operator.process(&self, data)
   }
}

impl Default for Node {
   #[inline]
   fn default() -> Node {
      Node::new(
         Box::new(NoneOp::new()),
         vec![],
         vec![],
         0
      )
   }
}


trait Operator where Self: fmt::Debug {
   fn new() -> Self where Self: Sized;

   fn process(&self, node: &Node, data: &[Data]) -> Data;
}


#[derive(Debug)]
struct NoneOp { }

impl Operator for NoneOp {
   #[inline]
   fn new() -> Self {
      NoneOp { }
   }

   #[inline]
   fn process(&self, _: &Node, _: &[Data]) -> Data {
      NONE
   }
}


#[derive(Debug)]
struct AddOp { }

impl Operator for AddOp {
   #[inline]
   fn new() -> Self {
      AddOp { }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      let in1 = node.input(data, 0);
      let in2 = node.input(data, 1);

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
struct JoinOp { }

impl JoinOp {
   #[inline]
   fn process_two(&self, node: &Node, data: &[Data]) -> Data {
      let in1 = node.input(data, 0);
      let in2 = node.input(data, 1);

      match (in1, in2) {
         (&Data::I64(v1), &Data::I64(v2)) => Data::I64I64((v1, v2)),

         _ => NONE
      }
   }

   #[inline]
   fn process_three(&self, node: &Node, data: &[Data]) -> Data {
      let in1 = node.input(data, 0);
      let in2 = node.input(data, 1);
      let in3 = node.input(data, 2);

      match (in1, in2, in3) {
         (&Data::U8(v1), &Data::U8(v2), &Data::U8(v3)) => Data::U8U8U8((v1, v2, v3)),

         _ => NONE
      }
   }
}

impl Operator for JoinOp {
   #[inline]
   fn new() -> Self {
      JoinOp { }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      match node.len() {
         2 => self.process_two(node, data),
         3 => self.process_three(node, data),
         _ => Data::None
      }
   }
}


#[derive(Debug)]
struct ListOp { }

impl ListOp {
   #[inline]
   fn create_poly_list(&self, node: &Node, data: &[Data]) -> Data {
      let mut result = Vec::with_capacity(node.len());

      for i in 0..node.len() {
         let input = node.input(data, i);

         match input {
            &Data::Poly(ref poly) => result.push((*poly).clone()),
            _ => {}
         }
      }

      Data::VPoly(result)
   }
}

impl Operator for ListOp {
   #[inline]
   fn new() -> Self {
      ListOp { }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      let in1 = node.input(data, 0);

      match in1 {
         &Data::Poly(_) => self.create_poly_list(node, data),
         _ => NONE
      }
   }
}


#[derive(Debug)]
struct PolyOp { }

impl Operator for PolyOp {
   #[inline]
   fn new() -> Self {
      PolyOp { }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      let points = node.input(data, 0);
      let color = node.input(data, 1);

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
struct LayerOp { }

impl Operator for LayerOp {
   #[inline]
   fn new() -> Self {
      LayerOp { }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      let polys_data = node.input(data, 0);

      match polys_data {
         &Data::VPoly(_) => {
            Data::Layer(Box::new(Layer {}))
         },
         _ => NONE
      }
   }
}


#[derive(Debug)]
struct ArtboardOp {
   list_node: ListOp,
}

impl Operator for ArtboardOp {
   #[inline]
   fn new() -> Self {
      ArtboardOp {
         list_node: ListOp::new()
      }
   }

   #[inline]
   fn process(&self, node: &Node, data: &[Data]) -> Data {
      self.list_node.process(node, data)
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

      let state = [
         Data::None,
         Data::I64(self.frame),
         Data::VVI64I64(source),
         Data::I64I64((self.frame, 0)),
         Data::None,
      ];

      let add = Node::new(
         Box::new(AddOp::new()),
         vec![NONE, NONE],
         vec![Some(2), Some(3)],
         4
      );

      let destination = add.process(&state);

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

         // Data::None at index 0, frame number at index 1
         index_map.insert("frame", 1);

         for (i, node_id) in all_tables.keys().enumerate() {
            let index = i + NODE_INDEX_OFFSET;
            println!("NODE {} {}", index, node_id);
            index_map.insert(node_id.as_str(), index);
         }

         println!("");

         let mut state = create_state(all_tables.len());

         let mut nodes = Vec::new();

         for (i, (node_id, value)) in all_tables.iter().enumerate() {
            match value {
               &toml::Value::Table(ref node_table) => {
                  let index = i + NODE_INDEX_OFFSET;
                  let node = process_node_table(
                     node_id, index, node_table, &index_map, &mut state
                  );

                  match node {
                     Some(node) => nodes.push(node),
                     None => {}
                  }
               },
               _ => {
                  println!("`{}` is not a table ", node_id);
               }
            }
         }


         println!("STATE {:?}", state);

         println!("");

         let nodes = execution_sort(nodes);

         for (i, node) in nodes.iter().enumerate() {
            println!("[{} / {}] {:?}", node.index, i, node);
         }

         println!("");
      },
      None => {
         println!("parse errors: {:?}", parser.errors);
      }
   }
}

fn create_state(nodes_len: usize) -> Vec<Data> {
   let data_len = nodes_len + NODE_INDEX_OFFSET;

   let mut state = Vec::with_capacity(data_len);

   for _ in 0..data_len {
      state.push(Data::None);
   }

   state
}

fn execution_sort(mut nodes: Vec<Node>) -> Vec<Node> {
   let len = nodes.len();

   let ordering = topological_ordering(&nodes);

   let mut positions: Vec<usize> = repeat(0).take(len).collect();

   for (position, order) in ordering.iter().enumerate() {
      positions[*order] = position;
   }

   let mut result = default_node_vec(len);

   for j in 0..len {
      let i = len - j - 1;

      let node = nodes.pop().unwrap();

      result[positions[i]] = node;
   }

   result
}

fn default_node_vec(len: usize) -> Vec<Node> {
   let mut nodes = Vec::with_capacity(len);

   for _ in 0..len {
      nodes.push(Node::default());
   }

   nodes
}


fn topological_ordering(nodes: &Vec<Node>) -> Vec<usize> {
   let connections = connections_map(&nodes);

   let mut ordering = Vec::new();

   let mut to_visit = Vec::new();

   let mut processed = HashSet::new();

   for root in 0..nodes.len() {
      if !processed.contains(&root) {

         to_visit.push((false, root));
      }

      while let Some((parent, current)) = to_visit.pop() {
         if processed.contains(&current) {
            break;
         }

         if parent {
            ordering.push(current);
            processed.insert(current);
         } else {
            to_visit.push((true, current));

            for child in connections[current].iter() {
               if !processed.contains(child) {
                  to_visit.push((false, *child));
               }
            }
         }
      }
   }

   ordering.reverse();

   ordering
}

fn connections_map(nodes: &Vec<Node>) -> Vec<Vec<usize>> {
   let positions = positions_map(nodes);

   let mut connections: Vec<Vec<usize>> = repeat(Vec::new()).take(nodes.len()).collect();

   for (i, node) in nodes.iter().enumerate() {
      for inlet in &node.inlets {
         match inlet {
            &Some(in_index) => {
               match positions.get(&in_index) {
                  Some(node_index) => {
                     connections[*node_index].push(i);
                  },
                  _ => {}
               }
            },
            _ => {}
         }
      }
   }

   connections
}


fn positions_map(nodes: &Vec<Node>) -> HashMap<usize, usize> {
   let mut positions = HashMap::new();

   for (i, node) in nodes.iter().enumerate() {
      positions.insert(node.index, i);
   }

   positions
}


fn process_node_table(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
   index_map: &HashMap<&str, usize>,
   state: &mut Vec<Data>,
) -> Option<Node> {

   let node_type = extract_node_type(node_id, node_table);

   println!("TYPE: {}", node_type);

   println!("{:?}", node_table);

   match node_type.as_ref() {
      "add" => return create_node::<AddOp>(node_id, node_index, node_table, index_map),
      "join" => return create_node::<JoinOp>(node_id, node_index, node_table, index_map),
      "list" => return create_node::<ListOp>(node_id, node_index, node_table, index_map),

      "poly" => return create_node::<PolyOp>(node_id, node_index, node_table, index_map),
      "layer" => return create_node::<LayerOp>(node_id, node_index, node_table, index_map),
      "artboard" => return create_node::<ArtboardOp>(node_id, node_index, node_table, index_map),
      _ => {},
   }

   let data = extract_table_data(node_id, node_table);
   state[node_index] = data;

   None
}


fn create_node<T: 'static + Operator>(
   node_id: &str, node_index: usize, node_table: &toml::Table, index_map: &HashMap<&str, usize>
) -> Option<Node> {
   let data_value = extract_data_value(node_id, node_table);

   println!("DATA {:?}", data_value);

   let (consts, inlets) = to_defaults(node_id, data_value, index_map);

   println!("consts: {:?}", consts);
   println!("inlets: {:?}", inlets);

   let operator = Box::new(T::new());

   Some(
      Node::new(operator, consts, inlets, node_index)
   )
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

   let mut consts = Vec::with_capacity(array.len());
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

            consts.push(Data::None);

         },
         None => {
            consts.push(
               extract_table_data(node_id, table)
            );

            inlets.push(None);
         }
      }
   }

   (consts, inlets)
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


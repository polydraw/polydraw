extern crate polydraw;
extern crate toml;

use std::fmt;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use std::mem::replace;

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


#[derive(Debug, Clone)]
pub struct Layer {
   pub polys: Vec<Box<Poly>>,
}

impl Layer {
   #[inline]
   pub fn new(polys: Vec<Box<Poly>>) -> Self {
      Layer {
         polys: polys,
      }
   }
}


type U8U8U8 = (u8, u8, u8);

type I64I64 = (i64, i64);
type VI64I64 = Vec<I64I64>;
type VVI64I64 = Vec<Vec<I64I64>>;

type PolyBox = Box<Poly>;
type VPolyBox = Vec<Box<Poly>>;

type LayerBox = Box<Layer>;
type VLayerBox = Vec<Box<Layer>>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
   VLayer(VLayerBox),
}

const NONE: Data = Data::None;

#[derive(Debug)]
struct Node {
   pub operator: Box<Operator>,
   pub consts: Vec<Data>,
   pub inlets: Vec<Option<(usize, usize)>>,
   pub slot: usize,
}

impl Node {
   #[inline]
   fn new(
      operator: Box<Operator>,
      consts: Vec<Data>,
      inlets: Vec<Option<(usize, usize)>>,
      slot: usize,
   ) -> Self {

      Node {
         operator: operator,
         consts: consts,
         inlets: inlets,
         slot: slot,
      }
   }

   #[inline]
   fn input(&self, state: &mut [Vec<Data>], slot: usize) -> Data {
      if let Some(option) = self.inlets.get(slot) {
         if let Some((data_index, slot_index)) = *option {
            let value = replace(&mut state[data_index][slot_index], Data::None);
            return value;
         }
      }

      match self.consts.get(slot) {
         Some(ref value) => (*value).clone(),
         None => Data::None
      }
   }

   #[inline]
   fn len(&self) -> usize {
      assert!(self.consts.len() == self.inlets.len());

      self.consts.len()
   }

   #[inline]
   fn process(&self, state: &mut [Vec<Data>]) {
      let data = self.operator.process(&self, state);

      let mut slots = &mut state[self.slot];

      for index in 1..slots.len() {
         slots[index] = data.clone();
      }

      slots[0] = data;
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

   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data;
}


#[derive(Debug)]
struct NoneOp { }

impl Operator for NoneOp {
   #[inline]
   fn new() -> Self {
      NoneOp { }
   }

   #[inline]
   fn process(&self, _: &Node, _: &mut [Vec<Data>]) -> Data {
      NONE
   }
}


#[derive(Debug)]
struct DataOp { }

impl Operator for DataOp {
   #[inline]
   fn new() -> Self {
      DataOp { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      node.input(state, 0)
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

trait Add<T1, T2> {
   fn add(v1: T1, v2: T2) -> Data;
}

impl Add<i64, i64> for (i64, i64) {
   #[inline]
   fn add(v1: i64, v2: i64) -> Data {
      Data::I64(v1 + v2)
   }
}

impl Add<f64, i64> for (f64, i64) {
   #[inline]
   fn add(v1: f64, v2: i64) -> Data {
      Data::F64(v1 + v2 as f64)
   }
}

impl Add<I64I64, i64> for (I64I64, i64) {
   #[inline]
   fn add(mut v1: I64I64, v2: i64) -> Data {
      v1.0 += v2;
      v1.1 += v2;

      Data::I64I64(v1)
   }
}

impl Add<VI64I64, I64I64> for (VI64I64, I64I64) {
   #[inline]
   fn add(mut v1: VI64I64, v2: I64I64) -> Data {
      for tuple in &mut v1 {
         tuple.0 += v2.0;
         tuple.1 += v2.1;
      }

      Data::VI64I64(v1)
   }
}

impl Add<VVI64I64, I64I64> for (VVI64I64, I64I64) {
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
struct JoinOp { }

impl JoinOp {
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

impl Operator for JoinOp {
   #[inline]
   fn new() -> Self {
      JoinOp { }
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
struct ListOp { }

impl ListOp {
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

impl Operator for ListOp {
   #[inline]
   fn new() -> Self {
      ListOp { }
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
struct PolyOp { }

impl Operator for PolyOp {
   #[inline]
   fn new() -> Self {
      PolyOp { }
   }

   #[inline]
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      let points = node.input(state, 0);
      let color = node.input(state, 1);

      match (points, color) {
         (Data::VI64I64(v1), Data::U8U8U8(v2)) => <(VI64I64, U8U8U8)>::create_poly(v1, v2),
         _ => NONE
      }
   }
}

trait PolyMake<T1, T2> {
   fn create_poly(v1: T1, v2: T2) -> Data;
}

impl PolyMake<VI64I64, U8U8U8> for (VI64I64, U8U8U8) {
   #[inline]
   fn create_poly(array: VI64I64, color: U8U8U8) -> Data {
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
   fn process(&self, node: &Node, state: &mut [Vec<Data>]) -> Data {
      self.list_node.process(node, state)
   }
}


struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
   nodes: Vec<Node>,
   state: Vec<Vec<Data>>,
   artboard_slot: usize,
}

impl NodeRenderer {
   #[inline]
   pub fn new() -> Self {
      let (nodes, state, artboard_slot) = parse(NODE_DEFS);
      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         nodes: nodes,
         state: state,
         artboard_slot: artboard_slot,
      }
   }
}

impl Renderer for NodeRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      self.renderer.init(width, height);
   }

   #[inline]
   fn render(&mut self, frame: &mut Frame) {
      self.state[1][0] = Data::I64(self.frame);

      for node in &self.nodes {
         node.process(&mut self.state);
      }

      let mut scene = Scene::new();

      if let Data::VLayer(ref artboard) = self.state[self.artboard_slot][0] {
         for layer in artboard {
            for poly in &layer.polys {
               scene.push(poly.clone());
            }
         }
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);

      self.frame += 1;
   }
}


fn parse(node_defs: &str) -> (Vec<Node>, Vec<Vec<Data>>, usize) {
   let mut parser = toml::Parser::new(node_defs);

   if let Some(all_tables) = parser.parse() {
      let mut slot_map = HashMap::new();

      // Data::None at slot 0, frame number at slot 1
      slot_map.insert("frame", 1);

      for (i, node_id) in all_tables.keys().enumerate() {
         let slot = i + NODE_INDEX_OFFSET;
         slot_map.insert(node_id.as_str(), slot);
      }

      let mut state = create_state(all_tables.len());

      let mut nodes = Vec::new();

      let mut artboard_slot = 0;

      for (i, (node_id, value)) in all_tables.iter().enumerate() {
         if let &toml::Value::Table(ref node_table) = value {
            let slot = i + NODE_INDEX_OFFSET;

            let result = process_node_table(
               node_id, slot, node_table, &slot_map, &mut state
            );

            if let Some((node, is_final)) = result {
               if is_final {
                  artboard_slot = node.slot;
               }

               nodes.push(node);
            }
         } else {
            panic!("`{}` is not a table ", node_id);
         }
      }

      let nodes = execution_sort(nodes);

      state[artboard_slot].push(Data::None);

      return (nodes, state, artboard_slot);
   }

   panic!("parse errors: {:?}", parser.errors);
}

fn create_state(nodes_len: usize) -> Vec<Vec<Data>> {
   let data_len = nodes_len + NODE_INDEX_OFFSET;

   let mut state = Vec::with_capacity(data_len);

   for _ in 0..data_len {
      state.push(Vec::new());
   }

   state[0].push(Data::None);

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
         if let &Some((in_index, _)) = inlet {
            if let Some(node_index) = positions.get(&in_index) {
               connections[*node_index].push(i);
            }
         }
      }
   }

   connections
}


fn positions_map(nodes: &Vec<Node>) -> HashMap<usize, usize> {
   let mut positions = HashMap::new();

   for (i, node) in nodes.iter().enumerate() {
      positions.insert(node.slot, i);
   }

   positions
}


fn process_node_table(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
   slot_map: &HashMap<&str, usize>,
   state: &mut Vec<Vec<Data>>,
) -> Option<(Node, bool)> {

   let node_type = extract_node_type(node_id, node_table);

   let operator = match node_type.as_ref() {
      "add" => create_operator::<AddOp>(),
      "join" => create_operator::<JoinOp>(),
      "list" => create_operator::<ListOp>(),

      "poly" => create_operator::<PolyOp>(),
      "layer" => create_operator::<LayerOp>(),
      "artboard" => create_operator::<ArtboardOp>(),

      _ => None,
   };

   if let Some(operator) = operator {
      let node = create_node(node_id, node_index, node_table, slot_map, operator, state);

      let is_final = node_type == "artboard";

      Some((node, is_final))
   } else {
      let node = create_data_node(node_id, node_index, node_table);

      Some((node, false))
   }
}


fn create_operator<T: 'static + Operator>() -> Option<Box<Operator>> {
   Some(Box::new(T::new()))
}


fn create_node(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
   slot_map: &HashMap<&str, usize>,
   operator: Box<Operator>,
   state: &mut Vec<Vec<Data>>,
) -> Node {

   let data_value = extract_data_value(node_id, node_table);

   let (consts, inlets) = to_defaults(node_id, data_value, slot_map, state);

   Node::new(operator, consts, inlets, node_index)
}


fn create_data_node(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
) -> Node {
   let data = extract_table_data(node_id, node_table);

   let operator = Box::new(DataOp::new());

   let consts = vec![data];
   let inlets = vec![None];

   Node::new(operator, consts, inlets, node_index)
}


fn extract_node_type<'a>(node_id: &str, node_table: &'a toml::Table) -> &'a str {
   if let Some(type_value) = node_table.get("type") {
      match type_value {
         &toml::Value::String(ref node_type) => node_type,
         _ => {
            panic!("node type not a string: {}", node_id);
         }
      }
   } else {
      panic!("node without type: {}", node_id);
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
   slot_map: &HashMap<&str, usize>,
   state: &mut Vec<Vec<Data>>,
) -> (Vec<Data>, Vec<Option<(usize, usize)>>) {

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

      match table.get("from") {
         Some(from) => {

            let in_id = match from {
               &toml::Value::String(ref in_id) => in_id,
               _ => {
                  panic!("From is not a string {:?}: {}", from, node_id);
               }
            };

            let slot = match slot_map.get::<str>(in_id) {
               Some(slot) => slot,
               _ => {
                  panic!("Unrecognized ID {:?}: {}", in_id, node_id);
               }
            };

            let subslot = state[*slot].len();

            state[*slot].push(Data::None);

            inlets.push(Some((*slot, subslot)));

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


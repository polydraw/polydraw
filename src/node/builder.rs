use std::mem::replace;
use std::iter::repeat;
use std::collections::{HashMap, HashSet};

use super::operator::{Operator, DataOperator, InputOperator};
use super::data::Data;
use super::node::{Node, IndexedInlet};


#[derive(Debug)]
pub enum Inlet {
   Source(String),
   Data(Data),
   None,
}


#[derive(Debug)]
pub enum NodeDef {
   Operator((String, Box<Operator>, Vec<Inlet>)),
   Data((String, Data)),
   Input(String),
   None,
}

impl NodeDef {
   fn key(&self) -> String {
      match self {
         &NodeDef::Operator((ref key, _, _)) => key.clone(),
         &NodeDef::Data((ref key, _)) => key.clone(),
         &NodeDef::Input(ref key) => key.clone(),

         _ => panic!(""),
      }
   }
}


pub struct FunctionDefs {
   pub name: String,
   pub argument_count: usize,
   pub node_defs: Vec<NodeDef>,
}

impl FunctionDefs {
   pub fn new(name: String, argument_count: usize) -> Self {
      FunctionDefs {
         name: name,
         argument_count: argument_count,
         node_defs: Vec::new(),
      }
   }
}


pub struct ProgramBuilder {
   pub defs: Vec<FunctionDefs>,
   pub anon_count: usize,
}


impl ProgramBuilder {
   pub fn new() -> Self {
      ProgramBuilder {
         defs: Vec::new(),
         anon_count: 0,
      }
   }

   pub fn function(&mut self, name: String, arguments: Vec<String>) {
      let mut def = FunctionDefs::new(name, arguments.len());

      for argument in arguments {
         def.node_defs.push(
            NodeDef::Input(String::from(argument))
         );
      }

      self.defs.push(def);
   }

   pub fn operator(&mut self, operator: Box<Operator>, node_id: String, inlets: Vec<Inlet>) {
      if let Some(last) = self.defs.last_mut() {
         last.node_defs.push(NodeDef::Operator((node_id, operator, inlets)));
      }
   }

   pub fn anonymous(&mut self, operator: Box<Operator>, inlets: Vec<Inlet>) -> Inlet {
      self.anon_count += 1;

      let node_id = format!("__{}__", self.anon_count);

      self.operator(operator, node_id.clone(), inlets);

      Inlet::Source(node_id)
   }

   pub fn data(&mut self, node_id: String, data: Data) {
      if let Some(last) = self.defs.last_mut() {
         last.node_defs.push(
            NodeDef::Data((String::from(node_id), data))
         );
      }
   }

   pub fn compile(self) -> Program {
      let mut functions = HashMap::new();

      for FunctionDefs {name, argument_count, node_defs} in self.defs {
         let function = compile_function(argument_count, node_defs);

         functions.insert(name, function);
      }

      Program::new(functions)
   }
}


pub struct Program {
   pub functions: HashMap<String, Function>,
}

impl Program {
   pub fn new(functions: HashMap<String, Function>) -> Self {
      Program {
         functions: functions,
      }
   }

   pub fn execute(&mut self, arguments: Vec<Data>) -> Data {
      self.execute_function(String::from("main"), arguments)
   }

   pub fn execute_function(&mut self, name: String, arguments: Vec<Data>) -> Data {
      match self.functions.remove(&name) {
         Some(mut function) => {
            function.push_arguments(arguments);

            for node in &function.nodes {
               node.process(self, &mut function.state);
            }

            let data = replace(&mut function.state[function.result_slot][0], Data::None);

            self.functions.insert(name, function);

            data
         },
         None => panic!("No {:?} function available", name),
      }
   }

   pub fn argument_count(&self, name: &str) -> Option<usize> {
      match self.functions.get(name) {
         Some(function) => Some(function.argument_count),
         None => None,
      }
   }
}


pub fn compile_function(argument_count: usize, node_defs: Vec<NodeDef>) -> Function {
   let mut slot_map = HashMap::new();

   let mut result_slot = 0;

   for (i, node_def) in node_defs.iter().enumerate() {
      let node_id = node_def.key();

      if &node_id == "result" {
         result_slot = i;
      }

      slot_map.insert(node_id, i);
   }

   let mut state = create_state(node_defs.len());

   let mut nodes = Vec::new();

   let mut i = 0;

   for node_def in node_defs {
      let node = node_from_def(i, node_def, &slot_map, &mut state);

      nodes.push(node);

      i += 1;
   }

   let nodes = execution_sort(nodes);

   state[result_slot].push(Data::None);

   Function::new(nodes, state, result_slot, argument_count)
}


pub struct Function {
   pub nodes: Vec<Node>,
   pub state: Vec<Vec<Data>>,
   pub result_slot: usize,
   pub argument_count: usize,
}


impl Function {
   pub fn new(
      nodes: Vec<Node>,
      state: Vec<Vec<Data>>,
      result_slot: usize,
      argument_count: usize
   ) -> Self {
      Function {
         nodes: nodes,
         state: state,
         result_slot: result_slot,
         argument_count: argument_count,
      }
   }

   fn push_arguments(&mut self, arguments: Vec<Data>) {
      if arguments.len() > self.argument_count {
         panic!("Function call with higher number");
      }

      let mut slot = 0;
      for data in arguments {
         self.push_single_argument(slot, data);
         slot += 1;
      }

      for rest in slot..self.argument_count {
         self.push_single_argument(rest, Data::None);
      }
   }

   fn push_single_argument(&mut self, slot: usize, data: Data) {
      if self.state[slot].len() > 0 {
         for i in 1..self.state[slot].len() {
            self.state[slot][i] = data.clone();
         }

         self.state[slot][0] = data;
      }
   }
}


pub fn create_state(nodes_len: usize) -> Vec<Vec<Data>> {
   let mut state = Vec::with_capacity(nodes_len);

   for _ in 0..nodes_len {
      state.push(Vec::new());
   }

   state
}


pub fn execution_sort(mut nodes: Vec<Node>) -> Vec<Node> {
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
         if let &IndexedInlet::Slot((in_index, _)) = inlet {
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


fn default_node_vec(len: usize) -> Vec<Node> {
   let mut nodes = Vec::with_capacity(len);

   for _ in 0..len {
      nodes.push(Node::default());
   }

   nodes
}


fn node_from_def(
   node_index: usize,
   node_def: NodeDef,
   slot_map: &HashMap<String, usize>,
   state: &mut Vec<Vec<Data>>,
) -> Node {
   match node_def {
      NodeDef::Data((_, data)) => {
         let operator = DataOperator::new();

         let indexed_inlets = vec![
            IndexedInlet::Data(data)
         ];

         Node::new(operator, indexed_inlets, node_index)
      },
      NodeDef::Operator((_, operator, inlets)) => {
         let indexed_inlets = node_sources(inlets, slot_map, state);

         Node::new(operator, indexed_inlets, node_index)
      },
      NodeDef::Input(_) => {
         let operator = InputOperator::new();

         Node::new(operator, vec![], node_index)
      },
      _ => panic!("")
   }
}


fn node_sources(
   inlets: Vec<Inlet>,
   slot_map: &HashMap<String, usize>,
   state: &mut Vec<Vec<Data>>,
) -> Vec<IndexedInlet> {

   let mut indexed_inlets = Vec::with_capacity(inlets.len());

   for inlet in inlets {
      match inlet {
         Inlet::Source(node_id) => {
            let slot = match slot_map.get::<str>(&node_id) {
               Some(slot) => slot,
               _ => {
                  panic!("Unrecognized ID {:?}", node_id);
               }
            };

            let subslot = state[*slot].len();

            state[*slot].push(Data::None);

            indexed_inlets.push(
               IndexedInlet::Slot((*slot, subslot))
            );
         },
         Inlet::Data(data) => {
            indexed_inlets.push(
               IndexedInlet::Data(data)
            );
         },
         Inlet::None => {
            indexed_inlets.push(
               IndexedInlet::Data(Data::None)
            );
         }
      }
   }

   indexed_inlets
}

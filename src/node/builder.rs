use std::mem::replace;
use std::iter::repeat;
use std::collections::{HashMap, HashSet};

use super::operator::{Operator, DataOp};
use super::data::Data;
use super::node::{Node, NodeRole};

pub const NODE_INDEX_OFFSET: usize = 1;


#[derive(Debug)]
pub enum Inlet<'a> {
   Source(&'a str),
   Data(Data),
   None,
}


#[derive(Debug)]
enum OwnedInlet {
   Source(String),
   Data(Data),
   None,
}


#[derive(Debug)]
pub enum IndexedInlet {
   Slot((usize, usize)),
   Data(Data),
   None,
}


#[derive(Debug)]
enum NodeDef {
   Operator((String, Box<Operator>, Vec<OwnedInlet>)),
   Data((String, Data)),
   None,
}

impl NodeDef {
   fn key(&self) -> String {
      match self {
         &NodeDef::Operator((ref key, _, _)) => key.clone(),
         &NodeDef::Data((ref key, _)) => key.clone(),

         _ => panic!(""),
      }
   }
}


pub struct NodeBuilder {
   node_defs: Vec<NodeDef>,
}


impl NodeBuilder {
   pub fn new() -> Self {
      NodeBuilder {
         node_defs: Vec::new(),
      }
   }

   pub fn operator<T: 'static + Operator>(&mut self, node_id: &str, inlets: Vec<Inlet>) {
      let owned = to_owned_inlet_vec(inlets);

      let operator = Box::new(T::new());

      self.node_defs.push(
         NodeDef::Operator((String::from(node_id), operator, owned))
      );
   }

   pub fn data(&mut self, node_id: &str, data: Data) {
      self.node_defs.push(
         NodeDef::Data((String::from(node_id), data))
      );
   }

   pub fn compile(&mut self) -> NodeScene {
      let mut slot_map = HashMap::new();

      // Data::frame number at slot 0
      slot_map.insert(String::from("frame"), 0);

      for (i, node_def) in self.node_defs.iter().enumerate() {
         let slot = i + NODE_INDEX_OFFSET;
         slot_map.insert(node_def.key(), slot);
      }



      let mut state = create_state(self.node_defs.len());

      let mut nodes = Vec::new();

      let mut artboard_slot = 0;

      let mut i = 0;

      for node_def_ref in self.node_defs.iter_mut() {
         let node_def = replace(node_def_ref, NodeDef::None);

         let slot = i + NODE_INDEX_OFFSET;

         i += 1;

         let node = node_from_def(
            slot, node_def, &slot_map, &mut state
         );

         if node.role() == NodeRole::Artboard {
            artboard_slot = node.slot;
         }

         nodes.push(node);
      }

      let nodes = execution_sort(nodes);

      state[artboard_slot].push(Data::None);

      NodeScene::new(nodes, state, artboard_slot)
   }
}


fn to_owned_inlet_vec(mut inlets: Vec<Inlet>) -> Vec<OwnedInlet> {
   let mut owned = Vec::with_capacity(inlets.len());

   for inlet in inlets.iter_mut() {
      let converted = replace(inlet, Inlet::None);

      owned.push(to_owned_inlet(converted));
   }

   owned
}

fn to_owned_inlet(inlet: Inlet) -> OwnedInlet {
   match inlet {
      Inlet::Source(source) => OwnedInlet::Source(String::from(source)),
      Inlet::Data(data) => OwnedInlet::Data(data),
      Inlet::None => OwnedInlet::None,
   }
}


pub struct NodeScene {
   pub nodes: Vec<Node>,
   pub state: Vec<Vec<Data>>,
   pub artboard_slot: usize,
}


impl NodeScene {
   pub fn new(nodes: Vec<Node>, state: Vec<Vec<Data>>, artboard_slot: usize) -> Self {
      NodeScene {
         nodes: nodes,
         state: state,
         artboard_slot: artboard_slot,
      }
   }
}


pub fn create_state(nodes_len: usize) -> Vec<Vec<Data>> {
   let data_len = nodes_len + NODE_INDEX_OFFSET;

   let mut state = Vec::with_capacity(data_len);

   for _ in 0..data_len {
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
         let operator = Box::new(DataOp::new());

         let consts = vec![data];
         let inlets = vec![None];

         Node::new(operator, consts, inlets, node_index)
      },
      NodeDef::Operator((_, operator, inlets)) => {
         let (consts, sources) = node_sources(inlets, slot_map, state);

         Node::new(operator, consts, sources, node_index)

      },
      _ => panic!("")
   }
}

fn node_sources(
   mut inlets: Vec<OwnedInlet>,
   slot_map: &HashMap<String, usize>,
   state: &mut Vec<Vec<Data>>,
) -> (Vec<Data>, Vec<Option<(usize, usize)>>) {

   let mut consts = Vec::with_capacity(inlets.len());
   let mut sources = Vec::with_capacity(inlets.len());

   for inlet in inlets.iter_mut() {
      let extracted = replace(inlet, OwnedInlet::None);

      match extracted {
         OwnedInlet::Source(node_id) => {
            let slot = match slot_map.get::<str>(&node_id) {
               Some(slot) => slot,
               _ => {
                  panic!("Unrecognized ID {:?}", node_id);
               }
            };

            let subslot = state[*slot].len();

            state[*slot].push(Data::None);

            sources.push(Some((*slot, subslot)));

            consts.push(Data::None);
         },
         OwnedInlet::Data(data) => {
            consts.push(data);

            sources.push(None);

         },
         OwnedInlet::None => {
            consts.push(Data::None);

            sources.push(None);
         }
      }
   }

   (consts, sources)
}


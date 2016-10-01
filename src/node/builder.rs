use std::mem::replace;
use std::iter::repeat;
use std::collections::{HashMap, HashSet};

use super::operator::{Operator, DataOperator};
use super::data::Data;
use super::node::{Node, NodeRole, IndexedInlet};

pub const NODE_INDEX_OFFSET: usize = 1;


#[derive(Debug)]
pub enum Inlet {
   Source(String),
   Data(Data),
   None,
}


#[derive(Debug)]
enum NodeDef {
   Operator((String, Box<Operator>, Vec<Inlet>)),
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
   anon_count: usize,
}


impl NodeBuilder {
   pub fn new() -> Self {
      NodeBuilder {
         node_defs: Vec::new(),
         anon_count: 0,
      }
   }

   pub fn operator<T: 'static + Operator>(&mut self, node_id: String, inlets: Vec<Inlet>) {
      let operator = Box::new(T::new());

      self.node_defs.push(
         NodeDef::Operator((node_id, operator, inlets))
      );
   }

   pub fn anonymous<T: 'static + Operator>(&mut self, inlets: Vec<Inlet>) -> String {
      self.anon_count += 1;

      let node_id = format!("__{}__", self.anon_count);

      self.operator::<T>(node_id.clone(), inlets);

      node_id
   }

   pub fn data(&mut self, node_id: String, data: Data) {
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
         let operator = Box::new(DataOperator::new());

         let indexed_inlets = vec![
            IndexedInlet::Data(data)
         ];

         Node::new(operator, indexed_inlets, node_index)
      },
      NodeDef::Operator((_, operator, inlets)) => {
         let indexed_inlets = node_sources(inlets, slot_map, state);

         Node::new(operator, indexed_inlets, node_index)

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


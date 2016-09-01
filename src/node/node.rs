use std::collections::{HashMap, HashSet};
use std::mem::replace;
use std::iter::repeat;

use super::operator::{Operator, NoneOp};
use super::data::Data;


pub const NODE_INDEX_OFFSET: usize = 1;


#[derive(Debug)]
pub struct Node {
   pub operator: Box<Operator>,
   pub consts: Vec<Data>,
   pub inlets: Vec<Option<(usize, usize)>>,
   pub slot: usize,
}

impl Node {
   #[inline]
   pub fn new(
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
   pub fn input(&self, state: &mut [Vec<Data>], slot: usize) -> Data {
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
   pub fn len(&self) -> usize {
      assert!(self.consts.len() == self.inlets.len());

      self.consts.len()
   }

   #[inline]
   pub fn process(&self, state: &mut [Vec<Data>]) {
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


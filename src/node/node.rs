use std::mem::replace;

use super::operator::{Operator, NoneOp};
use super::data::Data;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeRole {
   Data,
   Processor,
   Artboard,
}

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

   #[inline]
   pub fn role(&self) -> NodeRole {
      self.operator.role()
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


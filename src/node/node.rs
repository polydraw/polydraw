use std::mem::replace;

use super::operator::{Operator, NoneOperator};
use super::data::Data;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeRole {
   Data,
   Processor,
   Artboard,
}


#[derive(Debug)]
pub enum IndexedInlet {
   Slot((usize, usize)),
   Data(Data),
   None,
}


#[derive(Debug)]
pub struct Node {
   pub operator: Box<Operator>,
   pub inlets: Vec<IndexedInlet>,
   pub slot: usize,
}

impl Node {
   #[inline]
   pub fn new(
      operator: Box<Operator>,
      inlets: Vec<IndexedInlet>,
      slot: usize,
   ) -> Self {

      Node {
         operator: operator,
         inlets: inlets,
         slot: slot,
      }
   }

   #[inline]
   pub fn input(&self, state: &mut [Vec<Data>], slot: usize) -> Data {

      match &self.inlets[slot] {
         &IndexedInlet::Slot((data_index, slot_index)) => {
            replace(&mut state[data_index][slot_index], Data::None)
         },
         &IndexedInlet::Data(ref value) => (*value).clone(),
         &IndexedInlet::None => Data::None,
      }
   }

   #[inline]
   pub fn len(&self) -> usize {
      self.inlets.len()
   }

   #[inline]
   pub fn process(&self, state: &mut [Vec<Data>]) {
      let data = self.operator.process(&self, state);

      if let Some(data) = data {
         let mut slots = &mut state[self.slot];

         if slots.len() == 0 {
            return;
         }

         for index in 1..slots.len() {
            slots[index] = data.clone();
         }

         slots[0] = data;
      }
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
         Box::new(NoneOperator::new()),
         vec![],
         0
      )
   }
}


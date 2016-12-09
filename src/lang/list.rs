use std::any::TypeId;

use super::super::data::Empty;
use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::clone::{CloneRegistry, clone_value_ptr};
use super::drop::drop_value_ptr;
use super::execute::Executor;
use super::parser::FnRef;


pub fn create_list(arguments: Vec<&ValuePtr>, clone_registry: &CloneRegistry) -> ValuePtr {
   let mut list = Vec::new();

   for value_ptr in arguments.iter() {
      list.push(
         clone_value_ptr(value_ptr, clone_registry)
      );
   }

   ValuePtr::new(list)
}


pub fn each(arguments: Vec<&ValuePtr>, executor: Executor) -> Vec<ValuePtr> {
   if arguments.len() < 2
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vec![ValuePtr::new(Empty)];
   }

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut result = Vec::new();

   for value_ptr in list.iter() {
      let mut call_arguments = vec![value_ptr];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, call_arguments);

      let value = values.remove(0);

      for ptr in values {
         drop_value_ptr(&ptr, executor.drop_registry);
      }

      result.push(value);
   }

   vec![ValuePtr::new(result)]
}


pub fn each_with_last(arguments: Vec<&ValuePtr>, executor: Executor) -> Vec<ValuePtr> {
   if arguments.len() < 3
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vec![ValuePtr::new(Empty)];
   }

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut result = Vec::new();

   let mut last_item = arguments[2].clone();

   for value_ptr in list.iter() {
      let mut values = {
         let mut call_arguments = vec![value_ptr, &last_item];

         for arg in arguments[3..].iter() {
            call_arguments.push(*arg);
         }

         executor.execute_function(fn_ref, call_arguments)
      };

      let value = values.remove(0);

      for ptr in values {
         drop_value_ptr(&ptr, executor.drop_registry);
      }

      last_item = value.clone();

      result.push(value);
   }

   vec![ValuePtr::new(result)]
}


pub fn each_with_index(arguments: Vec<&ValuePtr>, executor: Executor) -> Vec<ValuePtr> {
   if arguments.len() < 2
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vec![ValuePtr::new(Empty)];
   }

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut result = Vec::new();

   for (index, value_ptr) in list.iter().enumerate() {
      let index_value = ValuePtr::new(index as i64);

      let mut call_arguments = vec![value_ptr, &index_value];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, call_arguments);

      let value = values.remove(0);

      for ptr in values {
         drop_value_ptr(&ptr, executor.drop_registry);
      }

      result.push(value);

      drop_value_ptr(&index_value, executor.drop_registry);
   }

   vec![ValuePtr::new(result)]
}


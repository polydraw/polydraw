use std::any::TypeId;

use super::super::data::Empty;
use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::clone::clone_value_ptr;
use super::drop::drop_value_ptr;
use super::execute::Executor;
use super::compiler::FnRef;


pub fn list(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   let mut list = Vec::new();

   for value_ptr in arguments.iter() {
      list.push(
         clone_value_ptr(value_ptr, executor.clone_registry)
      );
   }

   vecval!(list)
}


macro_rules! push_result {
   ($result:ident, $values:ident, $executor:ident) => {
      let value = $values.remove(0);

      for ptr in $values {
         drop_value_ptr(&ptr, $executor.drop_registry);
      }

      $result.push(value);
   }
}


pub fn each(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   if arguments.len() < 2
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vecval!(Empty);
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

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn each_with_last(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   if arguments.len() < 3
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vecval!(Empty);
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

   vecval!(result)
}


pub fn each_with_index(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   if arguments.len() < 2
      || TypeId::of::<ValuePtrList>() != arguments[0].type_id
      || TypeId::of::<FnRef>() != arguments[1].type_id {

      return vecval!(Empty);
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

      push_result!(result, values, executor);

      drop_value_ptr(&index_value, executor.drop_registry);
   }

   vecval!(result)
}


pub fn list_val_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = arguments[0];
   let right = value_ptr_as_ref!(arguments[1], ValuePtrList);

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, value_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_lst_val(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let right = arguments[1];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, right];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_lst_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[1], ValuePtrList);

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_lst_val_val(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = arguments[1];
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, middle, right];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_val_lst_val(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in middle.iter() {
      let call_arguments = vec![left, value_ptr, right];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_val_val_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = arguments[1];
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, middle, value_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}


pub fn list_lst_lst_val(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = arguments[2];

   let mut result = Vec::new();

   for (left_ptr, middle_ptr) in left.iter().zip(middle.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}

pub fn list_lst_val_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = arguments[1];
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle, right_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}

pub fn list_val_lst_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for (middle_ptr, right_ptr) in middle.iter().zip(right.iter()) {
      let call_arguments = vec![left, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}

pub fn list_lst_lst_lst(
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for ((left_ptr, middle_ptr), right_ptr) in left.iter().zip(middle.iter()).zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, call_arguments);

      push_result!(result, values, executor);
   }

   vecval!(result)
}

use std::any::TypeId;
use std::usize;

use data::Empty;

use lang::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use lang::clone::clone_value_ptr;
use lang::drop::drop_value_ptr;
use lang::execute::Executor;
use lang::compiler::FnRef;



macro_rules! push_result {
   ($result:ident, $values:ident) => {
      $result.push(
         if $values.len() == 1 {
            $values.remove(0)
         } else {
            ValuePtr::new($values)
         }
      );
   }
}


pub fn list(
   arguments: &[&ValuePtr],
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


pub fn call_lst_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let fn_refs = value_ptr_as_ref!(arguments[1], ValuePtrList);

   let mut result = Vec::new();

   for target_fn_ptr in fn_refs.iter() {
      let call_arguments = vec![arguments[0], target_fn_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn call_lst_fnp(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let arg_list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut call_arguments: Vec<&ValuePtr> = Vec::new();

   for arg in arg_list.iter() {
      call_arguments.push(arg);
   }

   for arg in arguments[2..].iter() {
      call_arguments.push(*arg);
   }

   executor.execute_function(fn_ref, &call_arguments)
}


pub fn each(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut result = Vec::new();

   for value_ptr in list.iter() {
      let mut call_arguments = vec![value_ptr];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn each_with_last(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   if arguments.len() < 3 {
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

         executor.execute_function(fn_ref, &call_arguments)
      };

      let value = if values.len() == 1 {
         values.remove(0)
      } else {
         ValuePtr::new(values)
      };

      last_item = value.clone();

      result.push(value);
   }

   vecval!(result)
}


pub fn each_with_index(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   let list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   let mut result = Vec::new();

   for (index, value_ptr) in list.iter().enumerate() {
      let index_value = ValuePtr::new(index as i64);

      let mut call_arguments = vec![value_ptr, &index_value];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);

      drop_value_ptr(&index_value, executor.drop_registry);
   }

   vecval!(result)
}


pub fn zip(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   if arguments.len() == 0 {
      return vecval!(Empty);
   }

   let mut lists = Vec::with_capacity(arguments.len());

   let mut min_len = usize::MAX;

   for value_ptr in arguments.iter() {
      if TypeId::of::<ValuePtrList>() == value_ptr.type_id {
         let list = value_ptr_as_ref!(value_ptr, ValuePtrList);

         if list.len() < min_len {
            min_len = list.len();
         }

         lists.push(list);
      } else {
         return vecval!(Empty);
      }
   }

   let mut result: ValuePtrList = Vec::new();

   for i in 0..min_len {
      let mut inner: ValuePtrList = Vec::new();

      for list in lists.iter() {
         inner.push(
            clone_value_ptr(&list[i], executor.clone_registry)
         );
      }

      result.push(ValuePtr::new(inner));
   }

   vecval!(result)
}


pub fn range(
   arguments: &[&ValuePtr],
   _: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {

   let start = value_ptr_as_ref!(arguments[0], i64);

   let end = value_ptr_as_ref!(arguments[1], i64);

   let mut result = Vec::new();

   for value in *start..*end {
      result.push(ValuePtr::new(value));
   }

   vecval!(result)
}


pub fn list_val_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = arguments[0];
   let right = value_ptr_as_ref!(arguments[1], ValuePtrList);

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, value_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_lst_val(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let right = arguments[1];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_lst_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {

   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[1], ValuePtrList);

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_lst_val_val(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = arguments[1];
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, middle, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_val_lst_val(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in middle.iter() {
      let call_arguments = vec![left, value_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_val_val_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = arguments[1];
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, middle, value_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


pub fn list_lst_lst_val(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = arguments[2];

   let mut result = Vec::new();

   for (left_ptr, middle_ptr) in left.iter().zip(middle.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}

pub fn list_lst_val_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = arguments[1];
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}

pub fn list_val_lst_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = arguments[0];
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for (middle_ptr, right_ptr) in middle.iter().zip(right.iter()) {
      let call_arguments = vec![left, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}

pub fn list_lst_lst_lst(
   arguments: &[&ValuePtr],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<ValuePtr> {
   let left = value_ptr_as_ref!(arguments[0], ValuePtrList);
   let middle = value_ptr_as_ref!(arguments[1], ValuePtrList);
   let right = value_ptr_as_ref!(arguments[2], ValuePtrList);

   let mut result = Vec::new();

   for ((left_ptr, middle_ptr), right_ptr) in left.iter().zip(middle.iter()).zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(result, values);
   }

   vecval!(result)
}


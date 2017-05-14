use std::usize;

use data::Empty;

use lang::variant::{Variant, VariantVec};
use lang::execute::Executor;
use lang::compiler::FnRef;



macro_rules! push_result {
   ($executor:ident, $result:ident, $values:ident) => {
      $result.push(
         if $values.len() == 1 {
            $values.remove(0)
         } else {
            $executor.registry.variant($values)
         }
      );
   }
}


pub fn list(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let mut list: Vec<Variant> = Vec::new();

   for variant in arguments.iter() {
      list.push(
         (*variant).clone()
      );
   }

   vecval!(executor, list)
}


pub fn call_lst_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let fn_refs = arguments[1].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for target_fn_ptr in fn_refs.iter() {
      let call_arguments = vec![arguments[0], target_fn_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn call_lst_fnp(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {
   let arg_list = arguments[0].as_ref::<VariantVec>();

   let fn_ref = arguments[1].as_ref::<FnRef>();

   let mut call_arguments: Vec<&Variant> = Vec::new();

   for arg in arg_list.iter() {
      call_arguments.push(arg);
   }

   for arg in arguments[2..].iter() {
      call_arguments.push(*arg);
   }

   executor.execute_function(fn_ref, &call_arguments)
}


pub fn each(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let list = arguments[0].as_ref::<VariantVec>();

   let fn_ref = arguments[1].as_ref::<FnRef>();

   let mut result = Vec::new();

   for variant in list.iter() {
      let mut call_arguments = vec![variant];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn each_with_last(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   if arguments.len() < 3 {
      return vecval!(executor, Empty);
   }

   let list = arguments[0].as_ref::<VariantVec>();

   let fn_ref = arguments[1].as_ref::<FnRef>();

   let mut result = Vec::new();

   let mut last_item = arguments[2].clone();

   for variant in list.iter() {
      let mut values = {
         let mut call_arguments = vec![variant, &last_item];

         for arg in arguments[3..].iter() {
            call_arguments.push(*arg);
         }

         executor.execute_function(fn_ref, &call_arguments)
      };

      let value = if values.len() == 1 {
         values.remove(0)
      } else {
         executor.registry.variant(values)
      };

      last_item = value.clone();

      result.push(value);
   }

   vecval!(executor, result)
}


pub fn each_with_index(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let list = arguments[0].as_ref::<VariantVec>();

   let fn_ref = arguments[1].as_ref::<FnRef>();

   let mut result = Vec::new();

   for (index, value_ptr) in list.iter().enumerate() {
      let index_value = executor.registry.variant(index as i64);

      let mut call_arguments = vec![value_ptr, &index_value];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn zip(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   if arguments.len() == 0 {
      return vecval!(executor, Empty);
   }

   let mut lists = Vec::with_capacity(arguments.len());

   let mut min_len = usize::MAX;

   for variant in arguments.iter() {
      if let Some(list) = variant.as_ref_checked::<VariantVec>() {
         if list.len() < min_len {
            min_len = list.len();
         }

         lists.push(list);
      } else {
         return vecval!(executor, Empty);
      }
   }

   let mut result: VariantVec = Vec::new();

   for i in 0..min_len {
      let mut inner: VariantVec = Vec::new();

      for list in lists.iter() {
         inner.push(
            list[i].clone()
         );
      }

      result.push(executor.registry.variant(inner));
   }

   vecval!(executor, result)
}


pub fn range(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let start = arguments[0].as_ref::<i64>();

   let end = arguments[1].as_ref::<i64>();

   let mut result = Vec::new();

   for value in *start..*end {
      result.push(executor.registry.variant(value));
   }

   vecval!(executor, result)
}


pub fn list_val_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {

   let left = arguments[0];
   let right = arguments[1].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, value_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_lst_val(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {

   let left = arguments[0].as_ref::<VariantVec>();
   let right = arguments[1];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_lst_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {

   let left = arguments[0].as_ref::<VariantVec>();
   let right = arguments[1].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_lst_val_val(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0].as_ref::<VariantVec>();
   let middle = arguments[1];
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in left.iter() {
      let call_arguments = vec![value_ptr, middle, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_val_lst_val(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0];
   let middle = arguments[1].as_ref::<VariantVec>();
   let right = arguments[2];

   let mut result = Vec::new();

   for value_ptr in middle.iter() {
      let call_arguments = vec![left, value_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_val_val_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0];
   let middle = arguments[1];
   let right = arguments[2].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for value_ptr in right.iter() {
      let call_arguments = vec![left, middle, value_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn list_lst_lst_val(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0].as_ref::<VariantVec>();
   let middle = arguments[1].as_ref::<VariantVec>();
   let right = arguments[2];

   let mut result = Vec::new();

   for (left_ptr, middle_ptr) in left.iter().zip(middle.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}

pub fn list_lst_val_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0].as_ref::<VariantVec>();
   let middle = arguments[1];
   let right = arguments[2].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for (left_ptr, right_ptr) in left.iter().zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}

pub fn list_val_lst_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0];
   let middle = arguments[1].as_ref::<VariantVec>();
   let right = arguments[2].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for (middle_ptr, right_ptr) in middle.iter().zip(right.iter()) {
      let call_arguments = vec![left, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}

pub fn list_lst_lst_lst(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   let left = arguments[0].as_ref::<VariantVec>();
   let middle = arguments[1].as_ref::<VariantVec>();
   let right = arguments[2].as_ref::<VariantVec>();

   let mut result = Vec::new();

   for ((left_ptr, middle_ptr), right_ptr) in left.iter().zip(middle.iter()).zip(right.iter()) {
      let call_arguments = vec![left_ptr, middle_ptr, right_ptr];

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}


pub fn all(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   if arguments.len() < 2 {
      return all_true(arguments, executor, fn_ref);
   }

   let list = arguments[0].as_ref::<VariantVec>();

   for value_ptr in list.iter() {
      let call_arguments = vec![value_ptr, arguments[1]];

      let mut values = executor.execute_function(
         &executor.builtin_fns.equal_ref,
         &call_arguments
      );

      let result = values.remove(0);

      let is_true = is_true(&result);

      if !is_true {
         return vecval!(executor, false);
      }
   }

   vecval!(executor, true)
}


pub fn all_true(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let list = arguments[0].as_ref::<VariantVec>();

   let true_value = executor.registry.variant(true);

   for value_ptr in list.iter() {
      let call_arguments = vec![value_ptr, &true_value];

      let mut values = executor.execute_function(
         &executor.builtin_fns.equal_ref,
         &call_arguments
      );

      let result = values.remove(0);

      let is_true = is_true(&result);

      if !is_true {
         return vecval!(executor, false);
      }
   }

   vecval!(executor, true)
}


pub fn any(
   arguments: &[&Variant],
   executor: &Executor,
   fn_ref: &FnRef
) -> Vec<Variant> {
   if arguments.len() < 2 {
      return any_true(arguments, executor, fn_ref);
   }

   let list = arguments[0].as_ref::<VariantVec>();

   for value_ptr in list.iter() {
      let call_arguments = vec![value_ptr, arguments[1]];

      let mut values = executor.execute_function(
         &executor.builtin_fns.equal_ref,
         &call_arguments
      );

      let result = values.remove(0);

      let is_true = is_true(&result);

      if is_true {
         return vecval!(executor, true);
      }
   }

   vecval!(executor, false)
}


pub fn any_true(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let list = arguments[0].as_ref::<VariantVec>();

   let true_value = executor.registry.variant(true);

   for value_ptr in list.iter() {
      let call_arguments = vec![value_ptr, &true_value];

      let mut values = executor.execute_function(
         &executor.builtin_fns.equal_ref,
         &call_arguments
      );

      let result = values.remove(0);

      let is_true = is_true(&result);

      if is_true {
         return vecval!(executor, true);
      }
   }

   vecval!(executor, false)
}


fn is_true(variant: &Variant) -> bool {
   if let Some(val) = variant.as_ref_checked::<bool>() {
      *val
   } else {
      false
   }
}


pub fn repeat(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {

   let count = arguments[0].as_ref::<i64>();

   if *count < 0 {
      return vecval!(executor, Empty);
   }

   let fn_ref = arguments[1].as_ref::<FnRef>();

   let mut result = Vec::new();

   for index in 0..(*count) as usize {
      let index_value = executor.registry.variant(index as i64);

      let mut call_arguments = vec![&index_value];

      for arg in arguments[2..].iter() {
         call_arguments.push(*arg);
      }

      let mut values = executor.execute_function(fn_ref, &call_arguments);

      push_result!(executor, result, values);
   }

   vecval!(executor, result)
}

use super::value_ptr::{ValuePtr, VoidPtr, ValuePtrList};
use super::execute::Executor;
use super::compiler::FnRef;



pub fn call_list_fn(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let arg_list = value_ptr_as_ref!(arguments[0], ValuePtrList);

   let fn_ref = value_ptr_as_ref!(arguments[1], FnRef);

   call(arguments, arg_list, fn_ref, executor)
}


pub fn call_fn_list(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let fn_ref = value_ptr_as_ref!(arguments[0], FnRef);

   let arg_list = value_ptr_as_ref!(arguments[1], ValuePtrList);

   call(arguments, arg_list, fn_ref, executor)
}


fn call(
   arguments: &[&ValuePtr],
   arg_list: &ValuePtrList,
   fn_ref: &FnRef,
   executor: &Executor
) -> Vec<ValuePtr> {

   let mut call_arguments: Vec<&ValuePtr> = Vec::new();

   for arg in arg_list.iter() {
      call_arguments.push(arg);
   }

   for arg in arguments[2..].iter() {
      call_arguments.push(*arg);
   }

   executor.execute_function(fn_ref, &call_arguments)
}


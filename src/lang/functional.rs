use super::value_ptr::{ValuePtr, VoidPtr};
use super::execute::Executor;
use super::compiler::FnRef;


pub fn call(
   mut arguments: Vec<&ValuePtr>,
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let fn_ref = value_ptr_as_ref!(arguments.remove(0), FnRef);

   executor.execute_function(fn_ref, arguments)
}

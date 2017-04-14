use data::Empty;

use lang::value_ptr::ValuePtr;
use lang::execute::Executor;
use lang::compiler::FnRef;
use lang::clone::clone_value_ptr;


pub fn if_(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let len = arguments.len();

   if len < 2 {
      return vecval!(Empty);
   }

   for i in 0..len / 2 {
      if let Some(guard) = arguments[i*2].as_ref_checked::<bool>() {
         if *guard {
            return vec![
               clone_value_ptr(arguments[i*2+1], executor.clone_registry)
            ];
         }
      } else {
         return vecval!(Empty);
      }
   }

   if len % 2 == 1 {
      return vec![
         clone_value_ptr(arguments[len-1], executor.clone_registry)
      ];
   }

   vecval!(Empty)
}

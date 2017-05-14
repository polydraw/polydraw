use data::Empty;

use lang::variant::Variant;
use lang::execute::Executor;
use lang::compiler::FnRef;


pub fn if_(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {
   let len = arguments.len();

   if len < 2 {
      return vecval!(executor, Empty);
   }

   for i in 0..len / 2 {
      if let Some(guard) = arguments[i*2].as_ref_checked::<bool>() {
         if *guard {
            return vec![arguments[i*2+1].clone()];
         }
      } else {
         return vecval!(executor, Empty);
      }
   }

   if len % 2 == 1 {
      return vec![arguments[len-1].clone()];
   }

   vecval!(executor, Empty)
}

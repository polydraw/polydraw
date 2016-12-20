

macro_rules! vecval {
   ($value:expr) => {
      vec![::lang::value_ptr::ValuePtr::new($value)]
   }
}


macro_rules! wrap_2_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(
         arguments: &[&::lang::value_ptr::ValuePtr],
         _: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::value_ptr::ValuePtr> {

         vecval!(
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
               unsafe { ::std::mem::transmute(arguments[1].data) },
            )
         )
      }
   }
}


macro_rules! wrap_3_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(
         arguments: &[&::lang::value_ptr::ValuePtr],
         _: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::value_ptr::ValuePtr> {

         vecval!(
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
               unsafe { ::std::mem::transmute(arguments[1].data) },
               unsafe { ::std::mem::transmute(arguments[2].data) },
            )
         )
      }
   }
}


macro_rules! wrap_4_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(
         arguments: &[&::lang::value_ptr::ValuePtr],
         _: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::value_ptr::ValuePtr> {

         vecval!(
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
               unsafe { ::std::mem::transmute(arguments[1].data) },
               unsafe { ::std::mem::transmute(arguments[2].data) },
               unsafe { ::std::mem::transmute(arguments[3].data) },
            )
         )
      }
   }
}

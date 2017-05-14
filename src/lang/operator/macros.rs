

macro_rules! vecval {
   ($executor:ident, $value:expr) => {
      vec![$executor.registry.variant($value)]
   }
}


macro_rules! wrap_1_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(
         arguments: &[&::lang::Variant],
         executor: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::Variant> {
         vecval!(
            executor,
            $func(
               unsafe { ::std::mem::transmute(arguments[0].data) },
            )
         )
      }
   }
}


macro_rules! wrap_2_arg {
   ($name:ident, $func:ident) => {
      pub fn $name(
         arguments: &[&::lang::Variant],
         executor: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::Variant> {
         vecval!(
            executor,
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
         arguments: &[&::lang::Variant],
         executor: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::Variant> {
         vecval!(
            executor,
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
         arguments: &[&::lang::Variant],
         executor: &::lang::execute::Executor,
         _: &::lang::compiler::FnRef
      ) -> Vec<::lang::Variant> {
         vecval!(
            executor,
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

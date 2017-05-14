use std::mem;

use sys::ft::FreeType;
use data::Empty;

use super::compiler::{Program, CompiledFn, CallArgType, ArgTemplate, FnRef};
use super::variant::{Variant, VariantVec};
use super::operator::{BuiltinFns, TypeFnMap};
use super::parser::FnType;
use super::registry::TypeRegistry;


pub fn execute_program(
   program: &Program,
   arguments: Vec<Variant>,
   builtin_fns: &BuiltinFns,
   registry: &TypeRegistry,
   freetype: &FreeType,
) -> Vec<Variant> {
   let mut arg_refs = Vec::new();

   for arg in arguments.iter() {
      arg_refs.push(arg);
   }

   let executor = Executor::new(
      &program.compiled_fns,
      builtin_fns,
      &program.consts,
      registry,
      freetype,
   );

   let fn_ref = FnRef::defined(program.main_index);

   execute_compiled_function(
      &fn_ref,
      &arg_refs,
      &executor,
   )
}


pub fn execute_builtin_function(
   fn_ref: &FnRef,
   args: &[&Variant],
   executor: &Executor,
) -> Vec<Variant> {
   match &executor.builtin_fns.fn_list[fn_ref.index] {
      &TypeFnMap::HMA1R1(ref map) => {
         if args.len() < 1 {
            vecval!(executor, Empty)
         } else if let Some(func) = map.get(
            args[0].type_id()
         ) {
            func(args, executor, fn_ref)
         } else {
            vecval!(executor, Empty)
         }
      },
      &TypeFnMap::HMA2R1(ref map) => {
         if args.len() < 2 {
            vecval!(executor, Empty)
         } else if let Some(func) = map.get(
            &(*args[0].type_id(), *args[1].type_id())
         ) {
            func(args, executor, fn_ref)
         } else {
            vecval!(executor, Empty)
         }
      },
      &TypeFnMap::HMA3R1(ref map) => {
         if args.len() < 3 {
            vecval!(executor, Empty)
         } else if let Some(func) = map.get(
            &(*args[0].type_id(), *args[1].type_id(), *args[2].type_id())
         ) {
            func(args, executor, fn_ref)
         } else {
            vecval!(executor, Empty)
         }
      },
      &TypeFnMap::HMA4R1(ref map) => {
         if args.len() < 4 {
            vecval!(executor, Empty)
         } else if let Some(func) = map.get(
            &(*args[0].type_id(), *args[1].type_id(), *args[2].type_id(), *args[3].type_id())
         ) {
            func(args, executor, fn_ref)
         } else {
            vecval!(executor, Empty)
         }
      },
      &TypeFnMap::CALL(ref func) => {
         func(args, executor, fn_ref)
      }
   }
}


fn execute_compiled_function(
   fn_ref: &FnRef,
   arguments: &[&Variant],
   executor: &Executor,
) -> Vec<Variant> {
   let mut stack: Vec<Variant> = Vec::new();

   let func = &executor.compiled_fns[fn_ref.index];

   let expanded_arguments = match expand_arguments(arguments, &func.template) {
      Some(expanded) => expanded,
      None => panic!("Expand arguments failed"),
   };

   for exec_fn in func.exec_lane.iter() {
      let value_ptr_list = {
         let mut argument_references = Vec::new();

         for call_arg in exec_fn.args.iter() {
            let reference = match call_arg.arg_type {
               CallArgType::Argument => expanded_arguments[call_arg.index],
               CallArgType::Const => &executor.consts[call_arg.index],
               CallArgType::Variable => &stack[call_arg.index],
            };

            argument_references.push(reference);
         }

         executor.execute_function(&exec_fn.fn_ref(), &argument_references)
      };

      for value_ptr in value_ptr_list {
         stack.push(value_ptr);
      }
   }

   let mut result = Vec::new();

   for call_arg in func.result_args.iter() {
      result.push(
         match call_arg.arg_type {
            CallArgType::Argument => {
               expanded_arguments[call_arg.index].clone()
            },
            CallArgType::Const => {
               executor.consts[call_arg.index].clone()
            },
            CallArgType::Variable => unsafe {
               mem::replace(stack.get_unchecked_mut(call_arg.index), Variant::null())
            },
         }
      );
   }

//   drop_value_ptr_vec(&stack, executor.drop_registry);

   result
}


pub struct Executor<'a> {
   pub compiled_fns: &'a Vec<CompiledFn>,
   pub builtin_fns: &'a BuiltinFns,
   pub consts: &'a Vec<Variant>,
   pub registry: &'a TypeRegistry,
   pub freetype: &'a FreeType,
}

impl<'a> Executor<'a> {
   pub fn new(
      compiled_fns: &'a Vec<CompiledFn>,
      builtin_fns: &'a BuiltinFns,
      consts: &'a Vec<Variant>,
      registry: &'a TypeRegistry,
      freetype: &'a FreeType,
   ) -> Self {
      Executor {
         compiled_fns: compiled_fns,
         builtin_fns: builtin_fns,
         consts: consts,
         registry: registry,
         freetype: freetype,
      }
   }

   pub fn execute_function(&self, fn_ref: &FnRef, arguments: &[&Variant]) -> Vec<Variant> {
      match fn_ref.fn_type {
         FnType::Builtin => {
            execute_builtin_function(
               fn_ref,
               arguments,
               self,
            )
         },
         FnType::Defined => {
            execute_compiled_function(
               fn_ref,
               arguments,
               self,
            )
         }
      }
   }
}


fn expand_arguments<'a>(
   arguments: &[&'a Variant],
   template: &Vec<ArgTemplate>,
) -> Option<Vec<&'a Variant>> {

   if arguments.len() != template.len() {
      return None;
   }

   let mut result = Vec::new();

   for (arg_template, arg) in template.iter().zip(arguments.iter()) {
      match arg_template {
         &ArgTemplate::Value => result.push(*arg),
         &ArgTemplate::List(ref inner_template) => {
            if let Some(list) = arg.as_ref_checked::<VariantVec>() {
               if !recurse_expand_list_value(
                  list,
                  inner_template,
                  &mut result,
               ) {
                  return None;
               }
            } else {
               return None;
            }
         }
      }
   }

   Some(result)
}


fn recurse_expand_list_value<'a>(
   list: &'a Vec<Variant>,
   template: &Vec<ArgTemplate>,
   result: &mut Vec<&'a Variant>,
) -> bool {

   if list.len() != template.len() {
      return false;
   }

   for (arg_template, arg) in template.iter().zip(list.iter()) {
      match arg_template {
         &ArgTemplate::Value => {
            result.push(arg)
         },
         &ArgTemplate::List(ref inner_template) => {
            if let Some(list) = arg.as_ref_checked::<VariantVec>() {
               if !recurse_expand_list_value(
                  list,
                  inner_template,
                  result,
               ) {
                  return false;
               }
            } else {
               return false;
            }
         }
      }
   }

   true
}


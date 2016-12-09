use std::mem;
use std::any::TypeId;

use super::super::data::Empty;

use super::compiler::{Program, CompiledFn, CallArgType, ArgTemplate};
use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::operator::{FnList, TypeFnMap};
use super::clone::{CloneRegistry, clone_value_ptr};
use super::drop::{DropRegistry, drop_value_ptr_vec};
use super::parser::{FnType, FnRef};


pub fn execute(
   program: &Program,
   arguments: Vec<ValuePtr>,
   builtin_fns: &FnList,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
) -> Vec<ValuePtr> {
   let result = {
      let mut arg_refs = Vec::new();

      for arg in arguments.iter() {
         arg_refs.push(arg);
      }

      execute_compiled_function(
         program.main_index,
         arg_refs,
         &program.compiled_fns,
         builtin_fns,
         &program.consts,
         clone_registry,
         drop_registry,
      )
   };

   drop_value_ptr_vec(&arguments, drop_registry);

   result
}


pub fn execute_builtin_function(
   index: usize,
   arguments: Vec<&ValuePtr>,
   compiled_fns: &Vec<CompiledFn>,
   builtin_fns: &FnList,
   consts: &Vec<ValuePtr>,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
) -> Vec<ValuePtr> {
   match &builtin_fns[index] {
      &TypeFnMap::HMA2R1(ref map) => {
         if let Some(func) = map.get(&(arguments[0].type_id, arguments[1].type_id)) {
            vec![func(arguments[0], arguments[1])]
         } else {
            vec![ValuePtr::new(Empty)]
         }
      },
      &TypeFnMap::ALR1(ref func) => {
         vec![func(arguments, clone_registry)]
      },
      &TypeFnMap::CALL(ref func) => {
         let executor = Executor::new(
            compiled_fns,
            builtin_fns,
            consts,
            clone_registry,
            drop_registry,
         );

         func(arguments, executor)
      }
   }
}


fn execute_compiled_function(
   index: usize,
   arguments: Vec<&ValuePtr>,
   compiled_fns: &Vec<CompiledFn>,
   builtin_fns: &FnList,
   consts: &Vec<ValuePtr>,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
) -> Vec<ValuePtr> {
   let mut stack: Vec<ValuePtr> = Vec::new();

   let func = &compiled_fns[index];

   let expanded_arguments = match expand_arguments(&arguments, &func.template) {
      Some(expanded) => expanded,
      None => panic!("Expand arguments failed"),
   };

   for exec_fn in func.exec_lane.iter() {
      let value_ptr_list = {
         let mut argument_references = Vec::new();

         for call_arg in exec_fn.args.iter() {
            let reference = match call_arg.arg_type {
               CallArgType::Argument => expanded_arguments[call_arg.index],
               CallArgType::Const => &consts[call_arg.index],
               CallArgType::Variable => &stack[call_arg.index],
            };

            argument_references.push(reference);
         }

         match exec_fn.fn_type {
            FnType::Builtin => {
               execute_builtin_function(
                  exec_fn.fn_index.index,
                  argument_references,
                  compiled_fns,
                  builtin_fns,
                  consts,
                  clone_registry,
                  drop_registry,
               )
            },
            FnType::Defined => {
               execute_compiled_function(
                  exec_fn.fn_index.index,
                  argument_references,
                  compiled_fns,
                  builtin_fns,
                  consts,
                  clone_registry,
                  drop_registry,
               )
            }
         }
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
               clone_value_ptr(expanded_arguments[call_arg.index], clone_registry)
            },
            CallArgType::Const => {
               clone_value_ptr(&consts[call_arg.index], clone_registry)
            },
            CallArgType::Variable => unsafe {
               mem::replace(stack.get_unchecked_mut(call_arg.index), ValuePtr::null())
            },
         }
      );
   }

   drop_value_ptr_vec(&stack, drop_registry);

   result
}


pub struct Executor<'a> {
   pub compiled_fns: &'a Vec<CompiledFn>,
   pub builtin_fns: &'a FnList,
   pub consts: &'a Vec<ValuePtr>,
   pub clone_registry: &'a CloneRegistry,
   pub drop_registry: &'a DropRegistry,
}

impl<'a> Executor<'a> {
   pub fn new(
      compiled_fns: &'a Vec<CompiledFn>,
      builtin_fns: &'a FnList,
      consts: &'a Vec<ValuePtr>,
      clone_registry: &'a CloneRegistry,
      drop_registry: &'a DropRegistry,
   ) -> Self {
      Executor {
         compiled_fns: compiled_fns,
         builtin_fns: builtin_fns,
         consts: consts,
         clone_registry: clone_registry,
         drop_registry: drop_registry,
      }
   }

   pub fn execute_function(&self, fn_ref: &FnRef, arguments: Vec<&ValuePtr>) -> Vec<ValuePtr> {
      match fn_ref.fn_type {
         FnType::Builtin => {
            execute_builtin_function(
               fn_ref.fn_index.index,
               arguments,
               self.compiled_fns,
               self.builtin_fns,
               self.consts,
               self.clone_registry,
               self.drop_registry,
            )
         },
         FnType::Defined => {
            execute_compiled_function(
               fn_ref.fn_index.index,
               arguments,
               self.compiled_fns,
               self.builtin_fns,
               self.consts,
               self.clone_registry,
               self.drop_registry,
            )
         }
      }
   }
}


fn expand_arguments<'a>(
   arguments: &Vec<&'a ValuePtr>,
   template: &Vec<ArgTemplate>,
) -> Option<Vec<&'a ValuePtr>> {

   if arguments.len() != template.len() {
      return None;
   }

   let mut result = Vec::new();

   for (arg_template, arg) in template.iter().zip(arguments.iter()) {
      match arg_template {
         &ArgTemplate::Value => result.push(*arg),
         &ArgTemplate::List(ref inner_template) => {
            if TypeId::of::<ValuePtrList>() == arg.type_id {
               let list = value_ptr_as_ref!(*arg, ValuePtrList);

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
   list: &'a Vec<ValuePtr>,
   template: &Vec<ArgTemplate>,
   result: &mut Vec<&'a ValuePtr>,
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
            if TypeId::of::<ValuePtrList>() == arg.type_id {
               let list = value_ptr_as_ref!(arg, ValuePtrList);

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


use std::mem;
use std::any::TypeId;

use super::super::data::Empty;

use super::compiler::{Program, CompiledFn, CallArgType, ArgTemplate, FnRef};
use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::operator::{FnList, TypeFnMap};
use super::clone::{CloneRegistry, clone_value_ptr};
use super::drop::{DropRegistry, drop_value_ptr_vec};
use super::debug::DebugRegistry;
use super::parser::FnType;


pub fn execute_program(
   program: &Program,
   arguments: Vec<ValuePtr>,
   builtin_fns: &FnList,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
   debug_registry: &DebugRegistry,
) -> Vec<ValuePtr> {
   let result = {
      let mut arg_refs = Vec::new();

      for arg in arguments.iter() {
         arg_refs.push(arg);
      }

      let executor = Executor::new(
         &program.compiled_fns,
         builtin_fns,
         &program.consts,
         clone_registry,
         drop_registry,
         debug_registry,
      );

      let fn_ref = FnRef::defined(program.main_index);

      execute_compiled_function(
         &fn_ref,
         arg_refs,
         &executor,
      )
   };

   drop_value_ptr_vec(&arguments, drop_registry);

   result
}


pub fn execute_builtin_function(
   fn_ref: &FnRef,
   args: Vec<&ValuePtr>,
   executor: &Executor,
) -> Vec<ValuePtr> {
   match &executor.builtin_fns[fn_ref.index] {
      &TypeFnMap::HMA1R1(ref map) => {
         if args.len() < 1 {
            vecval!(Empty)
         } else if let Some(func) = map.get(&args[0].type_id) {
            func(args, executor, fn_ref)
         } else {
            vecval!(Empty)
         }
      },
      &TypeFnMap::HMA2R1(ref map) => {
         if args.len() < 2 {
            vecval!(Empty)
         } else if let Some(func) = map.get(&(args[0].type_id, args[1].type_id)) {
            func(args, executor, fn_ref)
         } else {
            vecval!(Empty)
         }
      },
      &TypeFnMap::HMA3R1(ref map) => {
         if args.len() < 3 {
            vecval!(Empty)
         } else if let Some(func) = map.get(&(args[0].type_id, args[1].type_id, args[2].type_id)) {
            func(args, executor, fn_ref)
         } else {
            vecval!(Empty)
         }
      },
      &TypeFnMap::CALL(ref func) => {
         func(args, executor, fn_ref)
      }
   }
}


fn execute_compiled_function(
   fn_ref: &FnRef,
   arguments: Vec<&ValuePtr>,
   executor: &Executor,
) -> Vec<ValuePtr> {
   let mut stack: Vec<ValuePtr> = Vec::new();

   let func = &executor.compiled_fns[fn_ref.index];

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
               CallArgType::Const => &executor.consts[call_arg.index],
               CallArgType::Variable => &stack[call_arg.index],
            };

            argument_references.push(reference);
         }

         match exec_fn.fn_type {
            FnType::Builtin => {
               execute_builtin_function(
                  &FnRef::builtin(exec_fn.fn_index.index),
                  argument_references,
                  executor,
               )
            },
            FnType::Defined => {
               execute_compiled_function(
                  &FnRef::defined(exec_fn.fn_index.index),
                  argument_references,
                  executor,
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
               clone_value_ptr(expanded_arguments[call_arg.index], executor.clone_registry)
            },
            CallArgType::Const => {
               clone_value_ptr(&executor.consts[call_arg.index], executor.clone_registry)
            },
            CallArgType::Variable => unsafe {
               mem::replace(stack.get_unchecked_mut(call_arg.index), ValuePtr::null())
            },
         }
      );
   }

   drop_value_ptr_vec(&stack, executor.drop_registry);

   result
}


pub struct Executor<'a> {
   pub compiled_fns: &'a Vec<CompiledFn>,
   pub builtin_fns: &'a FnList,
   pub consts: &'a Vec<ValuePtr>,
   pub clone_registry: &'a CloneRegistry,
   pub drop_registry: &'a DropRegistry,
   pub debug_registry: &'a DebugRegistry,
}

impl<'a> Executor<'a> {
   pub fn new(
      compiled_fns: &'a Vec<CompiledFn>,
      builtin_fns: &'a FnList,
      consts: &'a Vec<ValuePtr>,
      clone_registry: &'a CloneRegistry,
      drop_registry: &'a DropRegistry,
      debug_registry: &'a DebugRegistry,
   ) -> Self {
      Executor {
         compiled_fns: compiled_fns,
         builtin_fns: builtin_fns,
         consts: consts,
         clone_registry: clone_registry,
         drop_registry: drop_registry,
         debug_registry: debug_registry,
      }
   }

   pub fn execute_function(&self, fn_ref: &FnRef, arguments: Vec<&ValuePtr>) -> Vec<ValuePtr> {
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


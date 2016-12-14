use std::usize;
use std::cmp::max;
use std::iter::repeat;
use std::collections::{HashMap, HashSet};
use std::fmt;

use super::value_ptr::ValuePtr;
use super::parser::{FnType, FnIndex, Argument, Function, Value};
use super::operator::FnList;
use super::clone::CloneRegistry;
use super::drop::DropRegistry;
use super::debug::DebugRegistry;
use super::execute::{execute_builtin_function, Executor};


#[derive(PartialEq, Clone, Debug)]
pub enum CallArgType {
   Argument,
   Const,
   Variable,
}

#[derive(Clone, Debug)]
pub struct CallArg {
   pub arg_type: CallArgType,
   pub index: usize,
}

impl CallArg {
   #[inline]
   pub fn argument(index: usize) -> Self {
      CallArg {
         arg_type: CallArgType::Argument,
         index: index,
      }
   }

   #[inline]
   pub fn const_(index: usize) -> Self {
      CallArg {
         arg_type: CallArgType::Const,
         index: index,
      }
   }

   #[inline]
   pub fn variable(index: usize) -> Self {
      CallArg {
         arg_type: CallArgType::Variable,
         index: index,
      }
   }
}


#[derive(Debug)]
pub struct ExecFn {
   pub fn_type: FnType,
   pub fn_index: FnIndex,
   pub args: Vec<CallArg>,
   pub target: usize,
}

impl ExecFn {
   #[inline]
   pub fn builtin(fn_index: FnIndex, args: Vec<CallArg>, target: usize) -> Self {
      ExecFn {
         fn_type: FnType::Builtin,
         fn_index: fn_index,
         args: args,
         target: target,
      }
   }

   #[inline]
   pub fn defined(fn_index: FnIndex, args: Vec<CallArg>, target: usize) -> Self {
      ExecFn {
         fn_type: FnType::Defined,
         fn_index: fn_index,
         args: args,
         target: target,
      }
   }
}


#[derive(Clone, Debug)]
pub struct FnRef {
   pub fn_type: FnType,
   pub index: usize,
}

impl FnRef {
   #[inline]
   pub fn builtin(index: usize) -> Self {
      FnRef {
         fn_type: FnType::Builtin,
         index: index,
      }
   }

   #[inline]
   pub fn defined(index: usize) -> Self {
      FnRef {
         fn_type: FnType::Defined,
         index: index,
      }
   }
}


#[derive(Debug)]
pub enum ArgTemplate {
   Value,
   List(Vec<ArgTemplate>)
}


#[derive(Debug)]
pub struct CompiledFn {
   pub exec_lane: Vec<ExecFn>,
   pub stack_size: usize,
   pub result_args: Vec<CallArg>,
   pub template: Vec<ArgTemplate>,
}

impl CompiledFn {
   #[inline]
   pub fn new(span: usize, arguments: &Vec<Argument>)-> Self {
      let mut template = Vec::new();

      arguments_to_template(arguments, &mut template);

      CompiledFn {
         exec_lane: Vec::new(),
         stack_size: 0,
         result_args: repeat(CallArg::argument(0)).take(span).collect(),
         template: template,
      }
   }
}


fn arguments_to_template(arguments: &Vec<Argument>, template: &mut Vec<ArgTemplate>) {
   for argument in arguments.iter() {
      match argument {
         &Argument::Name(_) => {
            template.push(ArgTemplate::Value);
         },
         &Argument::List(ref list) => {
            let mut inner = Vec::new();

            arguments_to_template(list, &mut inner);

            template.push(ArgTemplate::List(inner));
         }
      }
   }
}


pub type DefinedIndices<'a> = HashMap<&'a str, FnIndex>;

pub type BuiltinIndices = HashMap<&'static str, FnIndex>;


pub struct Program {
   pub compiled_fns: Vec<CompiledFn>,
   pub main_index: usize,
   pub consts: Vec<ValuePtr>,
}

impl Program {
   #[inline]
   pub fn new(
      compiled_fns: Vec<CompiledFn>,
      main_index: usize,
      consts: Vec<ValuePtr>,
   ) -> Self {
      Program {
         compiled_fns: compiled_fns,
         main_index: main_index,
         consts: consts,
      }
   }
}

pub fn compile_program (
   functions: &Vec<Function>,
   builtin_indices: &BuiltinIndices,
   builtin_fns: &FnList,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
   debug_registry: &DebugRegistry,
) -> Result<Program, String> {
   let defined_indices = try!(map_defined_indices(&functions));

   let mut main_index = usize::MAX;

   let mut consts: Vec<ValuePtr> = Vec::new();

   let mut compiled_fns = Vec::new();

   for (index, function) in functions.iter().enumerate() {
      compiled_fns.push(
         try!(compile_function(
            function,
            &mut consts,
            builtin_indices,
            builtin_fns,
            &defined_indices,
            clone_registry,
            drop_registry,
            debug_registry,
         ))
      );

      if function.name == "main" {
         main_index = index;
      }
   }

   if main_index == usize::MAX {
      Err("Function 'main' is not defined".to_string())
   } else {
      Ok(Program::new(compiled_fns, main_index, consts))
   }
}

fn map_defined_indices<'a>(
   functions: &'a Vec<Function>
) -> Result<DefinedIndices<'a>, String> {
   let mut defined_indices: DefinedIndices = HashMap::new();

   for (index, function) in functions.iter().enumerate() {
      let span = try!(function_span(function));
      defined_indices.insert(
         &function.name as &str,
         FnIndex::new(index, span)
      );
   }

   Ok(defined_indices)
}

fn function_span(function: &Function) -> Result<usize, String> {
   let mut max_index = usize::MIN;
   let mut total = 0;

   for assignment in function.assignments.iter() {
      for name in assignment.names.iter() {
         if let Some(index) = as_return_variable(name) {
            max_index = max(max_index, index);
            total += 1;
         }
      }
   }

   if max_index + 1 != total {
      Err(format!("Function '{}' missing return indices", function.name))
   } else {
      Ok(total)
   }
}

fn as_return_variable(name: &str) -> Option<usize> {
   if name.chars().next().unwrap() != '$' {
      return None;
   }

   if let Ok(index) = name[1..].parse::<usize>() {
      if index > 32 {
         None
      } else {
         Some(index)
      }
   } else {
      None
   }
}


fn compile_function(
   function: &Function,
   consts: &mut Vec<ValuePtr>,
   builtin_indices: &BuiltinIndices,
   builtin_fns: &FnList,
   defined_indices: &DefinedIndices,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
   debug_registry: &DebugRegistry,
) -> Result<CompiledFn, String> {
   let span = defined_indices[&function.name as &str].span;

   let mut compiled_fn = CompiledFn::new(span, &function.arguments.arguments);

   let mut variable_map: HashMap<&str, CallArg> = HashMap::new();

   for (index, argument) in function.flat_arguments.iter().enumerate() {
      variable_map.insert(argument as &str, CallArg::argument(index));
   }

   let ordering = try!(assignment_ordering(function));

   for index in ordering.iter() {
      let assignment = &function.assignments[*index];

      let (call_arg, span) = try!(compile_value(
         &mut compiled_fn,
         &variable_map,
         &assignment.value,
         consts,
         builtin_indices,
         builtin_fns,
         defined_indices,
         clone_registry,
         drop_registry,
         debug_registry,
      ));

      if span < assignment.names.len() {
         return Err(format!(
            "Assignment with more variables than values: '{}' in function '{}'",
            assignment.names.join(" "),
            function.name,
         ));
      }

      match call_arg.arg_type {
         CallArgType::Argument => {
            variable_map.insert(&assignment.names[0] as &str, call_arg);
         },
         CallArgType::Const => {
            variable_map.insert(&assignment.names[0] as &str, call_arg);
         },
         CallArgType::Variable => {
            for (i, name) in assignment.names.iter().enumerate() {
               variable_map.insert(name, CallArg::variable(call_arg.index + i));
            }
         },
      }
   }

   for (name, call_arg) in variable_map.iter() {
      if let Some(index) = as_return_variable(name) {
         compiled_fn.result_args[index] = call_arg.clone();
      }
   }

   Ok(compiled_fn)
}

fn compile_value(
   compiled_fn: &mut CompiledFn,
   variable_map: &HashMap<&str, CallArg>,
   value: &Value,
   consts: &mut Vec<ValuePtr>,
   builtin_indices: &BuiltinIndices,
   builtin_fns: &FnList,
   defined_indices: &DefinedIndices,
   clone_registry: &CloneRegistry,
   drop_registry: &DropRegistry,
   debug_registry: &DebugRegistry,
) -> Result<(CallArg, usize), String> {
   match value {
      &Value::Int(value) => push_const(consts, value),
      &Value::Float(value) => push_const(consts, value),
      &Value::Bool(value) => push_const(consts, value),
      &Value::String(ref value) => push_const(consts, (**value).clone()),
      &Value::List(ref list) => {
         let mut list_args = Vec::new();

         for value in list.iter() {
            let (compiled, _) = try!(compile_value(
               compiled_fn,
               variable_map,
               value,
               consts,
               builtin_indices,
               builtin_fns,
               defined_indices,
               clone_registry,
               drop_registry,
               debug_registry,
            ));

            list_args.push(compiled);
         }

         let fn_index = builtin_indices.get("list").unwrap().clone();

         let target = compiled_fn.stack_size;

         compiled_fn.stack_size += fn_index.span;
         compiled_fn.exec_lane.push(
            ExecFn::builtin(fn_index.clone(), list_args, target)
         );

         Ok((CallArg::variable(target), fn_index.span))
      },
      &Value::Name(ref name) => {
         Ok(((*variable_map.get(name as &str).unwrap()).clone(), 1))
      },
      &Value::FunctionRef(ref name) => {
         let fn_ref = if let Some(fn_index) = defined_indices.get(name as &str) {
            FnRef::defined(fn_index.index)
         } else if let Some(fn_index) = builtin_indices.get(name as &str) {
            FnRef::builtin(fn_index.index)
         } else {
            return Err(format!("Reference to unrecognized function '{}'", name));
         };

         push_const(consts, fn_ref)
      },
      &Value::Call(ref call) => {
         let mut consts_only = true;

         let mut call_args = Vec::with_capacity(call.arguments.len());

         for value in call.arguments.iter() {
            let (call_arg, _) = try!(compile_value(
               compiled_fn,
               variable_map,
               value,
               consts,
               builtin_indices,
               builtin_fns,
               defined_indices,
               clone_registry,
               drop_registry,
               debug_registry,
            ));

            if call_arg.arg_type != CallArgType::Const {
               consts_only = false;
            }

            call_args.push(call_arg);
         }

         let target = compiled_fn.stack_size;

         if let Some(fn_index) = defined_indices.get(&call.name as &str) {
            compiled_fn.stack_size += fn_index.span;
            compiled_fn.exec_lane.push(
               ExecFn::defined(fn_index.clone(), call_args, target)
            );

            Ok((CallArg::variable(target), fn_index.span))
         } else if let Some(fn_index) = builtin_indices.get(&call.name as &str) {
            if consts_only {
               let compiled_fns = Vec::new();

               let mut value_ptr_list = {
                  let mut argument_references = Vec::new();

                  for call_arg in call_args.iter() {
                     if call_arg.arg_type == CallArgType::Const {
                        argument_references.push(
                           &consts[call_arg.index]
                        );
                     }
                  }

                  let executor = Executor::new(
                     &compiled_fns,
                     builtin_fns,
                     consts,
                     clone_registry,
                     drop_registry,
                     debug_registry,
                  );

                  let fn_ref = FnRef::builtin(fn_index.index);

                  execute_builtin_function(
                     &fn_ref,
                     &argument_references,
                     &executor,
                  )
               };

               let pos = consts.len();

               consts.push(
                  if value_ptr_list.len() == 1 {
                     value_ptr_list.remove(0)
                  } else {
                     ValuePtr::new(value_ptr_list)
                  }
               );

               Ok((CallArg::const_(pos), 1))
            } else {
               compiled_fn.stack_size += fn_index.span;
               compiled_fn.exec_lane.push(
                  ExecFn::builtin(fn_index.clone(), call_args, target)
               );

               Ok((CallArg::variable(target), fn_index.span))
            }
         } else {
            Err(format!("Call of unrecognized function '{}'", call.name))
         }
      },
   }
}

fn push_const<T: 'static>(
   consts: &mut Vec<ValuePtr>,
   value: T
) -> Result<(CallArg, usize), String> where T: fmt::Debug {
   let pos = consts.len();

   consts.push(ValuePtr::new(value));

   Ok((CallArg::const_(pos), 1))
}

fn map_assignments<'a>(
   function: &'a Function
) -> HashMap<&'a str, usize> {
   let mut assignment_map: HashMap<&str, usize> = HashMap::new();

   for name in function.flat_arguments.iter() {
      assignment_map.insert(name as &str, 0);
   }

   for (outer, assignment) in function.assignments.iter().enumerate() {
      for name in assignment.names.iter() {
         assignment_map.insert(&name, outer + 1);
      }
   }

   assignment_map
}


type Graph = Vec<HashSet<usize>>;


fn assignment_ordering(function: &Function) -> Result<Vec<usize>, String> {
   let names_map = map_assignments(function);

   let len = function.assignments.len() + 1;
   let mut connections = init_graph(len);

   for (target, assignment) in function.assignments.iter().enumerate() {
      try!(connect_assignments(
         function,
         &mut connections,
         &assignment.value,
         target + 1,
         &names_map
      ));
   }

   let ordering_option = topological_ordering(&connections);

   if let Some(mut ordering) = ordering_option {
      let zero_index = ordering.iter().position(|&i| i == 0).unwrap();
      ordering.remove(zero_index);
      for value in ordering.iter_mut() {
         *value -= 1;
      }
      Ok(ordering)
   } else {
      Err(format!("Circular assignments found in function '{}'", function.name))
   }
}


fn init_graph(len: usize) -> Graph {
   repeat(HashSet::new()).take(len).collect()
}


fn connect_assignments(
   function: &Function,
   connections: &mut Vec<HashSet<usize>>,
   value: &Value,
   target: usize,
   names_map: &HashMap<&str, usize>
) -> Result<(), String> {
   match value {
      &Value::Name(ref name) => {
         if let Some(&source) = names_map.get(name as &str) {
            connections[source].insert(target);
         } else {
            return Err(
               format!(
                  "Variable '{}' not found in function '{}'", name, function.name
               )
            );
         }
      },
      &Value::Call(ref call) => {
         for value in call.arguments.iter() {
            try!(connect_assignments(function, connections, value, target, &names_map));
         }
      },
      &Value::List(ref list) => {
         for value in list.iter() {
            try!(connect_assignments(function, connections, value, target, &names_map));
         }
      },
      _ => {}
   }

   Ok(())
}


fn topological_ordering(
   connections: &Vec<HashSet<usize>>,
) -> Option<Vec<usize>> {
   let starting = starting_nodes(connections);

   let len = connections.len();

   let mut ordering = Vec::new();
   let mut visiting = Vec::new();
   let mut processed = HashSet::new();
   let mut parents = HashSet::new();

   for (root, _) in connections.iter().enumerate() {
      if !starting[root] {
         continue;
      }

      visiting.push(root);

      while let Some(index) = visiting.pop() {
         if processed.contains(&index) {
            continue;
         }

         if parents.contains(&index) {
            ordering.push(index);
            processed.insert(index);
            parents.remove(&index);
         } else {
            visiting.push(index);
            parents.insert(index);

            for child in connections[index].iter() {
               if !processed.contains(child) {
                  if parents.contains(child) {
                     return None;
                  }
                  visiting.push(*child);
               }
            }
         }
      }
   }

   if ordering.len() != len {
      return None;
   }

   ordering.reverse();

   Some(ordering)
}


fn starting_nodes(connections: &Vec<HashSet<usize>>) -> Vec<bool> {
   let mut starting: Vec<bool> = repeat(true).take(connections.len()).collect();

   for outgoing in connections.iter() {
      for target in outgoing.iter() {
         starting[*target] = false;
      }
   }

   starting
}


use sys::ft::FreeType;

use super::clone::{CloneRegistry, create_clone_registry};
use super::drop::{DropRegistry, create_drop_registry, drop_value_ptr_vec};
use super::debug::{DebugRegistry, create_debug_registry};
use super::compiler::{BuiltinIndices, Program, compile_program};
use super::operator::{BuiltinFns, register_builtin_fns};
use super::value_ptr::ValuePtr;
use super::tokenizer::tokenize;
use super::parser::parse;
use super::execute::execute_program;


pub struct Environment {
   pub clone_registry: CloneRegistry,
   pub drop_registry: DropRegistry,
   pub debug_registry: DebugRegistry,
   pub builtin_indices: BuiltinIndices,
   pub builtin_fns: BuiltinFns,
   pub freetype: FreeType,
}

impl Environment {
   pub fn new() -> Self {
      let (builtin_indices, builtin_fns) = register_builtin_fns();

      let freetype = FreeType::new();

      Environment {
         clone_registry: create_clone_registry(),
         drop_registry: create_drop_registry(),
         debug_registry: create_debug_registry(),
         builtin_indices: builtin_indices,
         builtin_fns: builtin_fns,
         freetype: freetype,
      }
   }

   pub fn compile_program(&self, source: &str) -> Result<Program, String> {
      let tokens = try!(tokenize(&source));

      let functions = try!(parse(tokens));

      compile_program(
         &functions,
         &self.builtin_indices,
         &self.builtin_fns,
         &self.clone_registry,
         &self.drop_registry,
         &self.debug_registry,
         &self.freetype,
      )
   }

   pub fn execute_program(
      &self,
      program: &Program,
      arguments: Vec<ValuePtr>
   ) -> Vec<ValuePtr> {
      execute_program(
         program,
         arguments,
         &self.builtin_fns,
         &self.clone_registry,
         &self.drop_registry,
         &self.debug_registry,
         &self.freetype,
      )
   }

   pub fn drop_result_contents(&self, result: &Vec<ValuePtr>) {
      drop_value_ptr_vec(result, &self.drop_registry);
   }

   pub fn drop_program_contents(&self, program: &Program) {
      drop_value_ptr_vec(&program.consts, &self.drop_registry);
   }
}


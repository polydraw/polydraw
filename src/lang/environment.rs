use sys::ft::FreeType;

use super::compiler::{BuiltinIndices, Program, compile_program};
use super::operator::{BuiltinFns, register_builtin_fns};
use super::variant::Variant;
use super::tokenizer::tokenize;
use super::parser::parse;
use super::execute::execute_program;
use super::registry::TypeRegistry;


pub struct Environment {
   pub registry: TypeRegistry,
   pub builtin_indices: BuiltinIndices,
   pub builtin_fns: BuiltinFns,
   pub freetype: FreeType,
}

impl Environment {
   pub fn new() -> Self {
      let (builtin_indices, builtin_fns) = register_builtin_fns();

      let freetype = FreeType::new();

      Environment {
         registry: TypeRegistry::new(),
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
         &self.registry,
         &self.freetype,
      )
   }

   pub fn execute_program(
      &self,
      program: &Program,
      arguments: Vec<Variant>
   ) -> Vec<Variant> {
      execute_program(
         program,
         arguments,
         &self.builtin_fns,
         &self.registry,
         &self.freetype,
      )
   }
}


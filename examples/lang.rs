extern crate polydraw;

use std::env;
use std::io::prelude::*;
use std::fs::File;

pub use polydraw::lang::{Environment, ValuePtr, debug_value_ptr};


fn main() {
   let filename = match env::args().nth(1) {
      Some(filename) => filename,
      None => {
         println!("No source file specified");
         return;
      }
   };

   let mut f = match File::open(&filename) {
      Ok(f) => f,
      Err(_) => {
         println!("Cannot open {}", &filename);
         return;
      }
   };

   let mut source = String::new();

   f.read_to_string(&mut source).unwrap();


   let environment = Environment::new();

   let program = match environment.compile_program(&source) {
      Ok(program) => program,
      Err(error) => {
         println!("Error: {}", error);
         return;
      }
   };

   let arguments = vec![
      ValuePtr::new(100_i64),
      ValuePtr::new(1600_i64 * 4_i64),
      ValuePtr::new(900_i64 * 4_i64),
   ];

   let result = environment.execute(&program, arguments);

   for (index, value_ptr) in result.iter().enumerate() {
      let debug = debug_value_ptr(value_ptr, &environment.debug_registry);
      println!("${} >> {}", index, debug);
   }

   environment.drop_result_contents(&result);

   environment.drop_program_contents(&program);
}



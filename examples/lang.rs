extern crate polydraw;

use std::env;
use std::io::prelude::*;
use std::fs::File;

use polydraw::node::{Data, NodeBuilder};
use polydraw::lang::{parse, compile, tokenize};


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

   match tokenize(&source) {
      Ok(tokens) => {
         match parse(tokens) {
            Ok(ast_list) => {
               let mut builder = NodeBuilder::new();

               let frame_index = builder.input(String::from("frame"));

               compile(&mut builder, ast_list);

               let mut program = builder.compile();

               program.input(frame_index, Data::Bool(false));

               let result = program.execute();

               println!(">> {:?}", result);
            },
            Err(err) => println!("{}", err),
         }
      },
      Err(err) => println!("{}", err)
   }
}



extern crate polydraw;

use polydraw::node::{Data, NodeBuilder};
use polydraw::lang::{parse, compile, tokenize};


pub const SOURCE: &'static str = "

double = frame * 2

result = <80 60> + double

";


fn main() {
   match tokenize(SOURCE) {
      Ok(tokens) => {
         match parse(tokens) {
            Ok(ast_list) => {
               let mut builder = NodeBuilder::new();

               let frame_index = builder.input(String::from("frame"));

               compile(&mut builder, ast_list);

               let mut program = builder.compile();

               program.input(frame_index, Data::Int(100));

               let result = program.execute();

               println!("{:?}", result);
            },
            Err(err) => println!("{}", err),
         }
      },
      Err(err) => println!("{}", err)
   }
}



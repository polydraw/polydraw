extern crate polydraw;

use std::env;
use std::io::prelude::*;
use std::fs::File;

use polydraw::Application;
use polydraw::lang::LangRenderer;


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

   let mut renderer = match LangRenderer::new(&source) {
      Ok(renderer) => renderer,
      Err(err) => {
         println!("Error: {}", err);
         return;
      }
   };

   Application::new()
      .renderer(&mut renderer)
      .title("Scene")
      .size(800, 450)
      .run();
}

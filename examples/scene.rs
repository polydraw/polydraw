extern crate polydraw;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::{self, File, DirEntry};
use std::path::Path;

use polydraw::Application;
use polydraw::lang::LangRenderer;


fn read_file(filename: &str) -> io::Result<String> {
   let mut source = String::new();

   let mut f = try!(File::open(&filename));

   try!(f.read_to_string(&mut source));

   Ok(source)
}


fn visit_dirs(dir: &Path, source: &mut String, cb: &Fn(&mut String, &DirEntry)) {
   for entry in fs::read_dir(dir).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_dir() {
         visit_dirs(&path, source, cb);
      } else {
         cb(source, &entry);
      }
   }
}


fn main() {
   let filename = match env::args().nth(1) {
      Some(filename) => filename,
      None => {
         println!("No source dir or file specified");
         return;
      }
   };

   let path = Path::new(&filename);

   let source = if path.is_file() {
      read_file(&filename).unwrap()

   } else if path.is_dir() {
      let mut source = String::new();

      visit_dirs(&path, &mut source, &|src: &mut String, entry: &DirEntry| {
         *src += &read_file(entry.path().to_str().unwrap()).unwrap();
         *src += "\n";
      });

      source
   } else {
      println!("Not a dir or a file");
      return;
   };

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

#![feature(path_ext)]

extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::fs::PathExt;
use std::io::BufWriter;
use std::path::Path;

fn main() {
   let out_dir = env::var("OUT_DIR").unwrap();
   let dest = Path::new(&out_dir);

   let egl_path = dest.join("egl_bindings.rs");

   if !egl_path.exists() {
      let mut egl_file = BufWriter::new(
         File::create(&egl_path).unwrap()
      );

      gl_generator::generate_bindings(
         gl_generator::GlobalGenerator,
         gl_generator::registry::Ns::Egl,
         gl_generator::Fallbacks::All,
         khronos_api::EGL_XML,
         vec![],
         "1.5", "core", &mut egl_file
      ).unwrap();
   }
}

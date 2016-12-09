use std::any::TypeId;

use data::{IntPoint, FloatPoint};
use devel::{DevelRenderer, Scene, SUBDIVISIONS, Poly};
use renderer::{Renderer};
use frame::Frame;
use draw::RGB;

use super::{Environment, ValuePtr, Program};
use super::value_ptr::{ValuePtrList, VoidPtr};


pub struct LangRenderer {
   renderer: DevelRenderer,
   frame: i64,
   environment: Environment,
   program: Program,
}


impl LangRenderer {
   #[inline]
   pub fn new(source: &str) -> Result<Self, String> {
      let environment = Environment::new();

       let program = match environment.compile_program(source) {
         Ok(program) => program,
         Err(error) => {
            return Err(error);
         }
      };

      Ok(LangRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         environment: environment,
         program: program,
      })
   }
}


impl Renderer for LangRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      self.renderer.init(width, height);
   }

   #[inline]
   fn render(&mut self, frame: &mut Frame) {
      let arguments = vec![
         ValuePtr::new(self.frame),
         ValuePtr::new(frame.width as i64 * SUBDIVISIONS),
         ValuePtr::new(frame.height as i64 * SUBDIVISIONS),
      ];

      let result = self.environment.execute(&self.program, arguments);

      let mut scene = Scene::new();


      for value_ptr in result.iter() {
         collect_polys(&mut scene, value_ptr);
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);


      self.environment.drop_result_contents(&result);

      self.frame += 1;
   }
}


impl Drop for LangRenderer {
   fn drop (&mut self) {
      self.environment.drop_program_contents(&self.program);
   }
}


fn collect_polys(scene: &mut Scene, value_ptr: &ValuePtr) {
   if TypeId::of::<ValuePtrList>() == value_ptr.type_id {
      let list = value_ptr_as_ref!(value_ptr, ValuePtrList);
      collect_polys_from_list(scene, list);
   }
}


fn collect_polys_from_list(scene: &mut Scene, value_ptrs: &Vec<ValuePtr>) {
   if value_ptrs.len() == 0 {
      return;
   }

   if is_point_list(value_ptrs) {
      let points = collect_poly_points(value_ptrs);

      let poly = Poly::new(vec![points], RGB::new(255, 255, 255));

      scene.push(Box::new(poly));
   }
}


fn is_point_list(value_ptrs: &Vec<ValuePtr>) -> bool {
   let int_point_ty = TypeId::of::<IntPoint>();
   let float_point_ty = TypeId::of::<FloatPoint>();

   for value_ptr in value_ptrs.iter() {
      if value_ptr.type_id != int_point_ty && value_ptr.type_id != float_point_ty {
         return false;
      }
   }

   true
}


fn collect_poly_points(value_ptrs: &Vec<ValuePtr>) -> Vec<IntPoint> {
   let mut points = Vec::new();

   for value_ptr in value_ptrs.iter() {
      if value_ptr.type_id == TypeId::of::<IntPoint>() {
         points.push(
            value_ptr_as_ref!(value_ptr, IntPoint).clone()
         );
      } else {
         points.push(
            value_ptr_as_ref!(value_ptr, FloatPoint).as_int()
         );
      }
   }

   points
}


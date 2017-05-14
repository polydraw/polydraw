use std::any::TypeId;

use data::{IntPoint, FloatPoint};
use devel::{DevelRenderer, Scene, SUBDIVISIONS, Poly};
use renderer::{Renderer};
use frame::Frame;
use draw::RGB;

use super::{Environment, Program};
use super::variant::{Variant, VariantVec};


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
         self.environment.registry.variant(self.frame),
         self.environment.registry.variant(frame.width as i64 * SUBDIVISIONS),
         self.environment.registry.variant(frame.height as i64 * SUBDIVISIONS),
      ];

      let result = self.environment.execute_program(&self.program, arguments);

      let mut scene = Scene::new();


      for value_ptr in result.iter() {
         collect_polys(&mut scene, value_ptr);
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);

      self.frame += 1;
   }
}


fn collect_polys(scene: &mut Scene, variant: &Variant) {
   if let Some(list) = variant.as_ref_checked::<VariantVec>() {
      collect_polys_from_list(scene, list);

   } else if let Some(poly) = variant.as_ref_checked::<Poly>() {
      scene.push(Box::new(poly.clone()));
   }
}


fn collect_polys_from_list(scene: &mut Scene, variants: &Vec<Variant>) {
   if variants.len() == 0 {
      return;
   }

   if is_point_list(variants) {
      let points = collect_poly_points(variants);

      let poly = Poly::new(vec![points], RGB::new(255, 255, 255));

      scene.push(Box::new(poly));

   } else {
      for variant in variants.iter() {
         if let Some(list) = variant.as_ref_checked::<VariantVec>() {
            collect_polys_from_list(scene, list);

         } else if let Some(poly) = variant.as_ref_checked::<Poly>() {
            scene.push(Box::new(poly.clone()));
         }
      }
   }
}


fn is_point_list(variants: &Vec<Variant>) -> bool {
   let int_point_ty = TypeId::of::<IntPoint>();
   let float_point_ty = TypeId::of::<FloatPoint>();

   for variant in variants.iter() {
      if *variant.type_id() != int_point_ty && *variant.type_id() != float_point_ty {
         return false;
      }
   }

   true
}


fn collect_poly_points(variants: &Vec<Variant>) -> Vec<IntPoint> {
   let mut points = Vec::new();

   for variant in variants.iter() {
      if *variant.type_id() == TypeId::of::<IntPoint>() {
         points.push(
            variant.as_ref::<IntPoint>().clone()
         );
      } else {
         points.push(
            variant.as_ref::<FloatPoint>().as_int()
         );
      }
   }

   points
}


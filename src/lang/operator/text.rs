use sys::ft::{Face, TextAlign};

use data::FloatPoint;

use lang::variant::Variant;
use lang::compiler::FnRef;
use lang::execute::Executor;


pub fn font_face(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {
   let path = arguments[0].as_ref::<String>();

   let face = executor.freetype.load_face(path);

   vecval!(executor, face)
}


pub fn text_fce_str_f64_fpt(
   arguments: &[&Variant], executor: &Executor, _: &FnRef
) -> Vec<Variant> {
   let face = arguments[0].as_ref::<Face>();
   let string = arguments[1].as_ref::<String>();
   let size = arguments[2].as_ref::<f64>();
   let origin = arguments[3].as_ref::<FloatPoint>();
   let text_align = alignment_argument(arguments);

   text_fce_str_f64_fpt_(
      executor, face, string, size, origin, text_align,
   )
}


fn alignment_argument(arguments: &[&Variant]) -> TextAlign {
   if arguments.len() < 5 {
      return TextAlign::Left;
   }

   let alignment = arguments[4].as_ref::<i64>();

   match *alignment {
      1 => TextAlign::Center,
      2 => TextAlign::Right,
      _ => TextAlign::Left,
   }
}


fn text_fce_str_f64_fpt_(
   executor: &Executor, face: &Face, string: &String, size: &f64, origin: &FloatPoint, text_align: TextAlign
) -> Vec<Variant> {

   let capped_size = if *size <= 0.0 { 0.0000001 } else { *size };
   let scale = capped_size / (2048.0 * 64.0);

   let text_contours = face.text(string, 20, text_align);

   let mut result = Vec::new();

   for char_contours in text_contours {
      let mut char_outer = Vec::new();

      for contour in char_contours.iter() {
         let mut char_inner = Vec::new();

         for point in contour.iter() {
            let transformed = FloatPoint::new(
               point.x * scale + origin.x,
               point.y * scale + origin.y,
            );
            char_inner.push(executor.registry.variant(transformed));
         }

         char_outer.push(executor.registry.variant(char_inner));
      }

      result.push(executor.registry.variant(char_outer));
   }

   vecval!(executor, result)
}


use sys::ft::Face;

use data::{FloatPoint, IntPoint};

use super::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use super::compiler::FnRef;
use super::execute::Executor;


pub fn font_face(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let path = value_ptr_as_ref!(arguments[0], String);

   let face = executor.freetype.load_face(path);

   vecval!(face)
}


fn text_fce_str_f64_fpt_(
   face: &Face, string: &String, size: &f64, origin: &FloatPoint
) -> ValuePtrList {

   let capped_size = if *size <= 0.0 { 0.0000001 } else { *size };
   let scale = capped_size / (2048.0 * 64.0);

   let text_contours = face.text(string, 20);

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
            char_inner.push(ValuePtr::new(transformed));
         }

         char_outer.push(ValuePtr::new(char_inner));
      }

      result.push(ValuePtr::new(char_outer));
   }

   result
}
wrap_4_arg!(text_fce_str_f64_fpt, text_fce_str_f64_fpt_);


fn text_fce_str_i64_fpt_(
   face: &Face, string: &String, size: &i64, origin: &FloatPoint
) -> ValuePtrList {
   text_fce_str_f64_fpt_(face, string, &(*size as f64), origin)
}
wrap_4_arg!(text_fce_str_i64_fpt, text_fce_str_i64_fpt_);


fn text_fce_str_f64_ipt_(
   face: &Face, string: &String, size: &f64, origin: &IntPoint
) -> ValuePtrList {
   text_fce_str_f64_fpt_(face, string, size, &origin.as_float())
}
wrap_4_arg!(text_fce_str_f64_ipt, text_fce_str_f64_ipt_);


fn text_fce_str_i64_ipt_(
   face: &Face, string: &String, size: &i64, origin: &IntPoint
) -> ValuePtrList {
   text_fce_str_f64_fpt_(face, string, &(*size as f64), &origin.as_float())
}
wrap_4_arg!(text_fce_str_i64_ipt, text_fce_str_i64_ipt_);


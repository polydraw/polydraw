use sys::ft::{Face, TextAlign};

use data::{FloatPoint, IntPoint};

use lang::value_ptr::{ValuePtr, ValuePtrList, VoidPtr};
use lang::compiler::FnRef;
use lang::execute::Executor;


pub fn font_face(
   arguments: &[&ValuePtr],
   executor: &Executor,
   _: &FnRef
) -> Vec<ValuePtr> {
   let path = value_ptr_as_ref!(arguments[0], String);

   let face = executor.freetype.load_face(path);

   vecval!(face)
}


pub fn text_fce_str_f64_fpt(
   arguments: &[&ValuePtr], _: &Executor, _: &FnRef
) -> Vec<ValuePtr> {
   let face = value_ptr_as_ref!(arguments[0], Face);
   let string = value_ptr_as_ref!(arguments[1], String);
   let size = value_ptr_as_ref!(arguments[2], f64);
   let origin = value_ptr_as_ref!(arguments[3], FloatPoint);
   let text_align = alignment_argument(arguments);

   text_fce_str_f64_fpt_(
      face, string, size, origin, text_align,
   )
}


pub fn text_fce_str_i64_fpt(
   arguments: &[&ValuePtr], _: &Executor, _: &FnRef
) -> ValuePtrList {
   let face = value_ptr_as_ref!(arguments[0], Face);
   let string = value_ptr_as_ref!(arguments[1], String);
   let size = value_ptr_as_ref!(arguments[2], i64);
   let origin = value_ptr_as_ref!(arguments[3], FloatPoint);
   let text_align = alignment_argument(arguments);

   text_fce_str_f64_fpt_(
      face, string, &(*size as f64), origin, text_align
   )
}


pub fn text_fce_str_f64_ipt(
   arguments: &[&ValuePtr], _: &Executor, _: &FnRef
) -> ValuePtrList {
   let face = value_ptr_as_ref!(arguments[0], Face);
   let string = value_ptr_as_ref!(arguments[1], String);
   let size = value_ptr_as_ref!(arguments[2], f64);
   let origin = value_ptr_as_ref!(arguments[3], IntPoint);
   let text_align = alignment_argument(arguments);

   text_fce_str_f64_fpt_(
      face, string, size, &origin.as_float(), text_align
   )
}


pub fn text_fce_str_i64_ipt(
   arguments: &[&ValuePtr], _: &Executor, _: &FnRef
) -> ValuePtrList {
   let face = value_ptr_as_ref!(arguments[0], Face);
   let string = value_ptr_as_ref!(arguments[1], String);
   let size = value_ptr_as_ref!(arguments[2], i64);
   let origin = value_ptr_as_ref!(arguments[3], IntPoint);
   let text_align = alignment_argument(arguments);

   text_fce_str_f64_fpt_(
      face, string, &(*size as f64), &origin.as_float(), text_align
   )
}


fn alignment_argument(arguments: &[&ValuePtr]) -> TextAlign {
   if arguments.len() < 5 {
      return TextAlign::Left;
   }

   let alignment = value_ptr_as_ref!(arguments[4], i64);

   match *alignment {
      1 => TextAlign::Center,
      2 => TextAlign::Right,
      _ => TextAlign::Left,
   }
}


fn text_fce_str_f64_fpt_(
   face: &Face, string: &String, size: &f64, origin: &FloatPoint, text_align: TextAlign
) -> Vec<ValuePtr> {

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
            char_inner.push(ValuePtr::new(transformed));
         }

         char_outer.push(ValuePtr::new(char_inner));
      }

      result.push(ValuePtr::new(char_outer));
   }

   vecval!(result)
}


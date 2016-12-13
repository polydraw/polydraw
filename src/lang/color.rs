use super::super::data::Rgb;
use super::value_ptr::ValuePtr;
use super::compiler::FnRef;
use super::execute::Executor;



fn rgb_f64_f64_f64_(r: &f64, g: &f64, b: &f64) -> Rgb {
   Rgb::new(r.round() as i64, g.round() as i64, b.round() as i64)
}
wrap_3_arg!(rgb_f64_f64_f64, rgb_f64_f64_f64_);


fn rgb_i64_f64_f64_(r: &i64, g: &f64, b: &f64) -> Rgb {
   Rgb::new(*r, g.round() as i64, b.round() as i64)
}
wrap_3_arg!(rgb_i64_f64_f64, rgb_i64_f64_f64_);


fn rgb_f64_i64_f64_(r: &f64, g: &i64, b: &f64) -> Rgb {
   Rgb::new(r.round() as i64, *g, b.round() as i64)
}
wrap_3_arg!(rgb_f64_i64_f64, rgb_f64_i64_f64_);


fn rgb_f64_f64_i64_(r: &f64, g: &f64, b: &i64) -> Rgb {
   Rgb::new(r.round() as i64, g.round() as i64, *b)
}
wrap_3_arg!(rgb_f64_f64_i64, rgb_f64_f64_i64_);


fn rgb_i64_i64_f64_(r: &i64, g: &i64, b: &f64) -> Rgb {
   Rgb::new(*r, *g, b.round() as i64)
}
wrap_3_arg!(rgb_i64_i64_f64, rgb_i64_i64_f64_);


fn rgb_i64_f64_i64_(r: &i64, g: &f64, b: &i64) -> Rgb {
   Rgb::new(*r, g.round() as i64, *b)
}
wrap_3_arg!(rgb_i64_f64_i64, rgb_i64_f64_i64_);


fn rgb_f64_i64_i64_(r: &f64, g: &i64, b: &i64) -> Rgb {
   Rgb::new(r.round() as i64, *g, *b)
}
wrap_3_arg!(rgb_f64_i64_i64, rgb_f64_i64_i64_);


fn rgb_i64_i64_i64_(r: &i64, g: &i64, b: &i64) -> Rgb {
   Rgb::new(*r, *g, *b)
}
wrap_3_arg!(rgb_i64_i64_i64, rgb_i64_i64_i64_);


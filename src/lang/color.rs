use draw::RGB;

use super::value_ptr::ValuePtr;
use super::compiler::FnRef;
use super::execute::Executor;



fn rgb_f64_f64_f64_(r: &f64, g: &f64, b: &f64) -> RGB {
   RGB::new(r.round() as u8, g.round() as u8, b.round() as u8)
}
wrap_3_arg!(rgb_f64_f64_f64, rgb_f64_f64_f64_);


fn rgb_i64_f64_f64_(r: &i64, g: &f64, b: &f64) -> RGB {
   RGB::new(*r as u8, g.round() as u8, b.round() as u8)
}
wrap_3_arg!(rgb_i64_f64_f64, rgb_i64_f64_f64_);


fn rgb_f64_i64_f64_(r: &f64, g: &i64, b: &f64) -> RGB {
   RGB::new(r.round() as u8, *g as u8, b.round() as u8)
}
wrap_3_arg!(rgb_f64_i64_f64, rgb_f64_i64_f64_);


fn rgb_f64_f64_i64_(r: &f64, g: &f64, b: &i64) -> RGB {
   RGB::new(r.round() as u8, g.round() as u8, *b as u8)
}
wrap_3_arg!(rgb_f64_f64_i64, rgb_f64_f64_i64_);


fn rgb_i64_i64_f64_(r: &i64, g: &i64, b: &f64) -> RGB {
   RGB::new(*r as u8, *g as u8, b.round() as u8)
}
wrap_3_arg!(rgb_i64_i64_f64, rgb_i64_i64_f64_);


fn rgb_i64_f64_i64_(r: &i64, g: &f64, b: &i64) -> RGB {
   RGB::new(*r as u8, g.round() as u8, *b as u8)
}
wrap_3_arg!(rgb_i64_f64_i64, rgb_i64_f64_i64_);


fn rgb_f64_i64_i64_(r: &f64, g: &i64, b: &i64) -> RGB {
   RGB::new(r.round() as u8, *g as u8, *b as u8)
}
wrap_3_arg!(rgb_f64_i64_i64, rgb_f64_i64_i64_);


fn rgb_i64_i64_i64_(r: &i64, g: &i64, b: &i64) -> RGB {
   RGB::new(*r as u8, *g as u8, *b as u8)
}
wrap_3_arg!(rgb_i64_i64_i64, rgb_i64_i64_i64_);


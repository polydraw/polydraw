use draw::RGB;



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


fn equal_rgb_rgb_(a: &RGB, b: &RGB) -> bool {
   *a == *b
}
wrap_2_arg!(equal_rgb_rgb, equal_rgb_rgb_);


fn unequal_rgb_rgb_(a: &RGB, b: &RGB) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_rgb_rgb, unequal_rgb_rgb_);


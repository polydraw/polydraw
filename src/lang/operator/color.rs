use draw::RGB;



fn rgb_(r: &f64, g: &f64, b: &f64) -> RGB {
   RGB::new(r.round() as u8, g.round() as u8, b.round() as u8)
}
wrap_3_arg!(rgb, rgb_);


fn equal_rgb_rgb_(a: &RGB, b: &RGB) -> bool {
   *a == *b
}
wrap_2_arg!(equal_rgb_rgb, equal_rgb_rgb_);


fn unequal_rgb_rgb_(a: &RGB, b: &RGB) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_rgb_rgb, unequal_rgb_rgb_);


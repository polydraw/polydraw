

fn add_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a + *b
}
wrap_2_arg!(add_f64_f64, add_f64_f64_);


fn multiply_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a * *b
}
wrap_2_arg!(multiply_f64_f64, multiply_f64_f64_);


fn subtract_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a - *b
}
wrap_2_arg!(subtract_f64_f64, subtract_f64_f64_);


fn divide_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a / *b
}
wrap_2_arg!(divide_f64_f64, divide_f64_f64_);


fn equal_f64_f64_(a: &f64, b: &f64) -> bool {
   *a == *b
}
wrap_2_arg!(equal_f64_f64, equal_f64_f64_);


fn unequal_f64_f64_(a: &f64, b: &f64) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_f64_f64, unequal_f64_f64_);


fn sin_f64_(a: &f64) -> f64 {
   let radians = a.to_radians();
   radians.sin()
}
wrap_1_arg!(sin_f64, sin_f64_);


fn cos_f64_(a: &f64) -> f64 {
   let radians = a.to_radians();
   radians.cos()
}
wrap_1_arg!(cos_f64, cos_f64_);


fn pow_f64_(a: &f64, b: &f64) -> f64 {
   a.powf(*b)
}
wrap_2_arg!(pow_f64, pow_f64_);

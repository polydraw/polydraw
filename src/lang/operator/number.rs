

fn add_i64_i64_(a: &i64, b: &i64) -> i64 {
   *a + *b
}
wrap_2_arg!(add_i64_i64, add_i64_i64_);

fn add_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a + *b
}
wrap_2_arg!(add_f64_f64, add_f64_f64_);

fn add_i64_f64_(a: &i64, b: &f64) -> f64 {
   *a as f64 + *b
}
wrap_2_arg!(add_i64_f64, add_i64_f64_);

fn add_f64_i64_(a: &f64, b: &i64) -> f64 {
   *a + *b as f64
}
wrap_2_arg!(add_f64_i64, add_f64_i64_);



fn multiply_i64_i64_(a: &i64, b: &i64) -> i64 {
   *a * *b
}
wrap_2_arg!(multiply_i64_i64, multiply_i64_i64_);

fn multiply_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a * *b
}
wrap_2_arg!(multiply_f64_f64, multiply_f64_f64_);

fn multiply_i64_f64_(a: &i64, b: &f64) -> f64 {
   *a as f64 * *b
}
wrap_2_arg!(multiply_i64_f64, multiply_i64_f64_);

fn multiply_f64_i64_(a: &f64, b: &i64) -> f64 {
   *a * *b as f64
}
wrap_2_arg!(multiply_f64_i64, multiply_f64_i64_);



fn subtract_i64_i64_(a: &i64, b: &i64) -> i64 {
   *a - *b
}
wrap_2_arg!(subtract_i64_i64, subtract_i64_i64_);

fn subtract_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a - *b
}
wrap_2_arg!(subtract_f64_f64, subtract_f64_f64_);

fn subtract_i64_f64_(a: &i64, b: &f64) -> f64 {
   *a as f64 - *b
}
wrap_2_arg!(subtract_i64_f64, subtract_i64_f64_);

fn subtract_f64_i64_(a: &f64, b: &i64) -> f64 {
   *a - *b as f64
}
wrap_2_arg!(subtract_f64_i64, subtract_f64_i64_);



fn divide_i64_i64_(a: &i64, b: &i64) -> f64 {
   *a as f64 / *b as f64
}
wrap_2_arg!(divide_i64_i64, divide_i64_i64_);

fn divide_f64_f64_(a: &f64, b: &f64) -> f64 {
   *a / *b
}
wrap_2_arg!(divide_f64_f64, divide_f64_f64_);

fn divide_i64_f64_(a: &i64, b: &f64) -> f64 {
   *a as f64 / *b
}
wrap_2_arg!(divide_i64_f64, divide_i64_f64_);

fn divide_f64_i64_(a: &f64, b: &i64) -> f64 {
   *a / *b as f64
}
wrap_2_arg!(divide_f64_i64, divide_f64_i64_);



fn equal_i64_i64_(a: &i64, b: &i64) -> bool {
   *a == *b
}
wrap_2_arg!(equal_i64_i64, equal_i64_i64_);

fn equal_f64_f64_(a: &f64, b: &f64) -> bool {
   *a == *b
}
wrap_2_arg!(equal_f64_f64, equal_f64_f64_);

fn equal_i64_f64_(a: &i64, b: &f64) -> bool {
   *a as f64 == *b
}
wrap_2_arg!(equal_i64_f64, equal_i64_f64_);

fn equal_f64_i64_(a: &f64, b: &i64) -> bool {
   *a == *b as f64
}
wrap_2_arg!(equal_f64_i64, equal_f64_i64_);



fn unequal_i64_i64_(a: &i64, b: &i64) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_i64_i64, unequal_i64_i64_);

fn unequal_f64_f64_(a: &f64, b: &f64) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_f64_f64, unequal_f64_f64_);

fn unequal_i64_f64_(a: &i64, b: &f64) -> bool {
   *a as f64 != *b
}
wrap_2_arg!(unequal_i64_f64, unequal_i64_f64_);

fn unequal_f64_i64_(a: &f64, b: &i64) -> bool {
   *a != *b as f64
}
wrap_2_arg!(unequal_f64_i64, unequal_f64_i64_);


fn sin_f64_(a: &f64) -> f64 {
   let radians = a.to_radians();
   radians.sin()
}
wrap_1_arg!(sin_f64, sin_f64_);

fn sin_i64_(a: &i64) -> f64 {
   sin_f64_(&(*a as f64))
}
wrap_1_arg!(sin_i64, sin_i64_);


fn cos_f64_(a: &f64) -> f64 {
   let radians = a.to_radians();
   radians.cos()
}
wrap_1_arg!(cos_f64, cos_f64_);

fn cos_i64_(a: &i64) -> f64 {
   cos_f64_(&(*a as f64))
}
wrap_1_arg!(cos_i64, cos_i64_);

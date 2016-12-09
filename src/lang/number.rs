use super::value_ptr::ValuePtr;
use super::compiler::FnRef;
use super::execute::Executor;


fn add_i64_(a: &i64, b: &i64) -> i64 {
   *a + *b
}
wrap_operator!(add_i64, add_i64_);

fn add_f64_(a: &f64, b: &f64) -> f64 {
   *a + *b
}
wrap_operator!(add_f64, add_f64_);



fn multiply_i64_(a: &i64, b: &i64) -> i64 {
   *a * *b
}
wrap_operator!(multiply_i64, multiply_i64_);

fn multiply_f64_(a: &f64, b: &f64) -> f64 {
   *a * *b
}
wrap_operator!(multiply_f64, multiply_f64_);



fn subtract_i64_(a: &i64, b: &i64) -> i64 {
   *a - *b
}
wrap_operator!(subtract_i64, subtract_i64_);

fn subtract_f64_(a: &f64, b: &f64) -> f64 {
   *a - *b
}
wrap_operator!(subtract_f64, subtract_f64_);



fn divide_i64_(a: &i64, b: &i64) -> i64 {
   *a / *b
}
wrap_operator!(divide_i64, divide_i64_);

fn divide_f64_(a: &f64, b: &f64) -> f64 {
   *a / *b
}
wrap_operator!(divide_f64, divide_f64_);


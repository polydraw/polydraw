use super::super::data::{FloatPoint, IntPoint};
use super::value_ptr::ValuePtr;
use super::compiler::FnRef;
use super::execute::Executor;



fn point_f64_f64_(a: &f64, b: &f64) -> FloatPoint {
   FloatPoint::new(*a, *b)
}
wrap_operator!(point_f64_f64, point_f64_f64_);

fn point_f64_i64_(a: &f64, b: &i64) -> FloatPoint {
   FloatPoint::new(*a, *b as f64)
}
wrap_operator!(point_f64_i64, point_f64_i64_);

fn point_i64_f64_(a: &i64, b: &f64) -> FloatPoint {
   FloatPoint::new(*a as f64, *b)
}
wrap_operator!(point_i64_f64, point_i64_f64_);

fn point_i64_i64_(a: &i64, b: &i64) -> IntPoint {
   IntPoint::new(*a, *b)
}
wrap_operator!(point_i64_i64, point_i64_i64_);



fn add_ipt_ipt_(a: &IntPoint, b: &IntPoint) -> IntPoint {
   IntPoint::new(a.x + b.x, a.y + b.y)
}
wrap_operator!(add_ipt_ipt, add_ipt_ipt_);

fn add_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x + b.x, a.y + b.y)
}
wrap_operator!(add_fpt_fpt, add_fpt_fpt_);

fn add_ipt_fpt_(a: &IntPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x as f64 + b.x, a.y as f64 + b.y)
}
wrap_operator!(add_ipt_fpt, add_ipt_fpt_);

fn add_fpt_ipt_(a: &FloatPoint, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(a.x + b.x as f64, a.y + b.y as f64)
}
wrap_operator!(add_fpt_ipt, add_fpt_ipt_);



fn add_ipt_i64_(a: &IntPoint, b: &i64) -> IntPoint {
   IntPoint::new(a.x + *b, a.y + *b)
}
wrap_operator!(add_ipt_i64, add_ipt_i64_);

fn add_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x + *b, a.y + *b)
}
wrap_operator!(add_fpt_f64, add_fpt_f64_);

fn add_ipt_f64_(a: &IntPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x as f64 + *b, a.y as f64 + *b)
}
wrap_operator!(add_ipt_f64, add_ipt_f64_);

fn add_fpt_i64_(a: &FloatPoint, b: &i64) -> FloatPoint {
   FloatPoint::new(a.x + *b as f64, a.y + *b as f64)
}
wrap_operator!(add_fpt_i64, add_fpt_i64_);



fn add_i64_ipt_(a: &i64, b: &IntPoint) -> IntPoint {
   IntPoint::new(*a + b.x, *a + b.y)
}
wrap_operator!(add_i64_ipt, add_i64_ipt_);

fn add_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a + b.x, *a + b.y)
}
wrap_operator!(add_f64_fpt, add_f64_fpt_);

fn add_f64_ipt_(a: &f64, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(*a + b.x as f64, *a + b.y as f64)
}
wrap_operator!(add_f64_ipt, add_f64_ipt_);

fn add_i64_fpt_(a: &i64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a as f64 + b.x, *a as f64 + b.y)
}
wrap_operator!(add_i64_fpt, add_i64_fpt_);



fn multiply_ipt_ipt_(a: &IntPoint, b: &IntPoint) -> IntPoint {
   IntPoint::new(a.x * b.x, a.y * b.y)
}
wrap_operator!(multiply_ipt_ipt, multiply_ipt_ipt_);

fn multiply_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x * b.x, a.y * b.y)
}
wrap_operator!(multiply_fpt_fpt, multiply_fpt_fpt_);

fn multiply_ipt_fpt_(a: &IntPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x as f64 * b.x, a.y as f64 * b.y)
}
wrap_operator!(multiply_ipt_fpt, multiply_ipt_fpt_);

fn multiply_fpt_ipt_(a: &FloatPoint, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(a.x * b.x as f64, a.y * b.y as f64)
}
wrap_operator!(multiply_fpt_ipt, multiply_fpt_ipt_);



fn multiply_ipt_i64_(a: &IntPoint, b: &i64) -> IntPoint {
   IntPoint::new(a.x * *b, a.y * *b)
}
wrap_operator!(multiply_ipt_i64, multiply_ipt_i64_);

fn multiply_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x * *b, a.y * *b)
}
wrap_operator!(multiply_fpt_f64, multiply_fpt_f64_);

fn multiply_ipt_f64_(a: &IntPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x as f64 * *b, a.y as f64 * *b)
}
wrap_operator!(multiply_ipt_f64, multiply_ipt_f64_);

fn multiply_fpt_i64_(a: &FloatPoint, b: &i64) -> FloatPoint {
   FloatPoint::new(a.x * *b as f64, a.y * *b as f64)
}
wrap_operator!(multiply_fpt_i64, multiply_fpt_i64_);



fn multiply_i64_ipt_(a: &i64, b: &IntPoint) -> IntPoint {
   IntPoint::new(*a * b.x, *a * b.y)
}
wrap_operator!(multiply_i64_ipt, multiply_i64_ipt_);

fn multiply_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a * b.x, *a * b.y)
}
wrap_operator!(multiply_f64_fpt, multiply_f64_fpt_);

fn multiply_f64_ipt_(a: &f64, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(*a * b.x as f64, *a * b.y as f64)
}
wrap_operator!(multiply_f64_ipt, multiply_f64_ipt_);

fn multiply_i64_fpt_(a: &i64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a as f64 * b.x, *a as f64 * b.y)
}
wrap_operator!(multiply_i64_fpt, multiply_i64_fpt_);



fn subtract_ipt_ipt_(a: &IntPoint, b: &IntPoint) -> IntPoint {
   IntPoint::new(a.x - b.x, a.y - b.y)
}
wrap_operator!(subtract_ipt_ipt, subtract_ipt_ipt_);

fn subtract_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x - b.x, a.y - b.y)
}
wrap_operator!(subtract_fpt_fpt, subtract_fpt_fpt_);

fn subtract_ipt_fpt_(a: &IntPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x as f64 - b.x, a.y as f64 - b.y)
}
wrap_operator!(subtract_ipt_fpt, subtract_ipt_fpt_);

fn subtract_fpt_ipt_(a: &FloatPoint, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(a.x - b.x as f64, a.y - b.y as f64)
}
wrap_operator!(subtract_fpt_ipt, subtract_fpt_ipt_);



fn subtract_ipt_i64_(a: &IntPoint, b: &i64) -> IntPoint {
   IntPoint::new(a.x - *b, a.y - *b)
}
wrap_operator!(subtract_ipt_i64, subtract_ipt_i64_);

fn subtract_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x - *b, a.y - *b)
}
wrap_operator!(subtract_fpt_f64, subtract_fpt_f64_);

fn subtract_ipt_f64_(a: &IntPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x as f64 - *b, a.y as f64 - *b)
}
wrap_operator!(subtract_ipt_f64, subtract_ipt_f64_);

fn subtract_fpt_i64_(a: &FloatPoint, b: &i64) -> FloatPoint {
   FloatPoint::new(a.x - *b as f64, a.y - *b as f64)
}
wrap_operator!(subtract_fpt_i64, subtract_fpt_i64_);



fn subtract_i64_ipt_(a: &i64, b: &IntPoint) -> IntPoint {
   IntPoint::new(*a - b.x, *a - b.y)
}
wrap_operator!(subtract_i64_ipt, subtract_i64_ipt_);

fn subtract_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a - b.x, *a - b.y)
}
wrap_operator!(subtract_f64_fpt, subtract_f64_fpt_);

fn subtract_f64_ipt_(a: &f64, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(*a - b.x as f64, *a - b.y as f64)
}
wrap_operator!(subtract_f64_ipt, subtract_f64_ipt_);

fn subtract_i64_fpt_(a: &i64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a as f64 - b.x, *a as f64 - b.y)
}
wrap_operator!(subtract_i64_fpt, subtract_i64_fpt_);



fn divide_ipt_ipt_(a: &IntPoint, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(a.x as f64 / b.x as f64, a.y as f64 / b.y as f64)
}
wrap_operator!(divide_ipt_ipt, divide_ipt_ipt_);

fn divide_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x / b.x, a.y / b.y)
}
wrap_operator!(divide_fpt_fpt, divide_fpt_fpt_);

fn divide_ipt_fpt_(a: &IntPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x as f64 / b.x, a.y as f64 / b.y)
}
wrap_operator!(divide_ipt_fpt, divide_ipt_fpt_);

fn divide_fpt_ipt_(a: &FloatPoint, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(a.x / b.x as f64, a.y / b.y as f64)
}
wrap_operator!(divide_fpt_ipt, divide_fpt_ipt_);



fn divide_ipt_i64_(a: &IntPoint, b: &i64) -> FloatPoint {
   FloatPoint::new(a.x as f64 / *b as f64, a.y as f64 / *b as f64)
}
wrap_operator!(divide_ipt_i64, divide_ipt_i64_);

fn divide_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x / *b, a.y / *b)
}
wrap_operator!(divide_fpt_f64, divide_fpt_f64_);

fn divide_ipt_f64_(a: &IntPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x as f64 / *b, a.y as f64 / *b)
}
wrap_operator!(divide_ipt_f64, divide_ipt_f64_);

fn divide_fpt_i64_(a: &FloatPoint, b: &i64) -> FloatPoint {
   FloatPoint::new(a.x / *b as f64, a.y / *b as f64)
}
wrap_operator!(divide_fpt_i64, divide_fpt_i64_);



fn divide_i64_ipt_(a: &i64, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(*a as f64 / b.x as f64, *a as f64 / b.y as f64)
}
wrap_operator!(divide_i64_ipt, divide_i64_ipt_);

fn divide_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a / b.x, *a / b.y)
}
wrap_operator!(divide_f64_fpt, divide_f64_fpt_);

fn divide_f64_ipt_(a: &f64, b: &IntPoint) -> FloatPoint {
   FloatPoint::new(*a / b.x as f64, *a / b.y as f64)
}
wrap_operator!(divide_f64_ipt, divide_f64_ipt_);

fn divide_i64_fpt_(a: &i64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a as f64 / b.x, *a as f64 / b.y)
}
wrap_operator!(divide_i64_fpt, divide_i64_fpt_);



fn polar_f64_f64_(radius: &f64, angle: &f64) -> FloatPoint {
   let radians = angle.to_radians();

   let x = *radius * radians.cos();
   let y = *radius * radians.sin();

   FloatPoint::new(x, y)
}
wrap_operator!(polar_f64_f64, polar_f64_f64_);

fn polar_i64_i64_(radius: &i64, angle: &i64) -> FloatPoint {
   polar_f64_f64_(&(*radius as f64), &(*angle as f64))
}
wrap_operator!(polar_i64_i64, polar_i64_i64_);

fn polar_f64_i64_(radius: &f64, angle: &i64) -> FloatPoint {
   polar_f64_f64_(radius, &(*angle as f64))
}
wrap_operator!(polar_f64_i64, polar_f64_i64_);

fn polar_i64_f64_(radius: &i64, angle: &f64) -> FloatPoint {
   polar_f64_f64_(&(*radius as f64), angle)
}
wrap_operator!(polar_i64_f64, polar_i64_f64_);


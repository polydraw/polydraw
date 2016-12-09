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


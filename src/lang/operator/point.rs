use std::f64::consts::PI;

use data::FloatPoint;


fn point_f64_f64_(a: &f64, b: &f64) -> FloatPoint {
   FloatPoint::new(*a, *b)
}
wrap_2_arg!(point_f64_f64, point_f64_f64_);


fn add_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x + b.x, a.y + b.y)
}
wrap_2_arg!(add_fpt_fpt, add_fpt_fpt_);

fn add_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x + *b, a.y + *b)
}
wrap_2_arg!(add_fpt_f64, add_fpt_f64_);


fn add_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a + b.x, *a + b.y)
}
wrap_2_arg!(add_f64_fpt, add_f64_fpt_);


fn multiply_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x * b.x, a.y * b.y)
}
wrap_2_arg!(multiply_fpt_fpt, multiply_fpt_fpt_);


fn multiply_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x * *b, a.y * *b)
}
wrap_2_arg!(multiply_fpt_f64, multiply_fpt_f64_);


fn multiply_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a * b.x, *a * b.y)
}
wrap_2_arg!(multiply_f64_fpt, multiply_f64_fpt_);


fn subtract_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x - b.x, a.y - b.y)
}
wrap_2_arg!(subtract_fpt_fpt, subtract_fpt_fpt_);


fn subtract_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x - *b, a.y - *b)
}
wrap_2_arg!(subtract_fpt_f64, subtract_fpt_f64_);


fn subtract_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a - b.x, *a - b.y)
}
wrap_2_arg!(subtract_f64_fpt, subtract_f64_fpt_);


fn divide_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(a.x / b.x, a.y / b.y)
}
wrap_2_arg!(divide_fpt_fpt, divide_fpt_fpt_);


fn divide_fpt_f64_(a: &FloatPoint, b: &f64) -> FloatPoint {
   FloatPoint::new(a.x / *b, a.y / *b)
}
wrap_2_arg!(divide_fpt_f64, divide_fpt_f64_);


fn divide_f64_fpt_(a: &f64, b: &FloatPoint) -> FloatPoint {
   FloatPoint::new(*a / b.x, *a / b.y)
}
wrap_2_arg!(divide_f64_fpt, divide_f64_fpt_);


fn equal_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> bool {
   a.x == b.x && a.y == b.y
}
wrap_2_arg!(equal_fpt_fpt, equal_fpt_fpt_);


fn unequal_fpt_fpt_(a: &FloatPoint, b: &FloatPoint) -> bool {
   a.x != b.x || a.y != b.y
}
wrap_2_arg!(unequal_fpt_fpt, unequal_fpt_fpt_);


fn polar_f64_f64_(radius: &f64, angle: &f64) -> FloatPoint {
   let radians = angle.to_radians();

   let x = *radius * radians.cos();
   let y = *radius * radians.sin();

   FloatPoint::new(x, y)
}
wrap_2_arg!(polar_f64_f64, polar_f64_f64_);


fn rotate_fpt_fpt_f64_(target: &FloatPoint, origin: &FloatPoint, angle: &f64) -> FloatPoint {

   let radians = angle.to_radians();

   let s = radians.sin();
   let c = radians.cos();

   let x = target.x - origin.x;
   let y = target.y - origin.y;

   FloatPoint::new(
      x * c - y * s + origin.x,
      x * s + y * c + origin.y,
   )
}
wrap_3_arg!(rotate_fpt_fpt_f64, rotate_fpt_fpt_f64_);


fn flip_x_fpt_f64_f64_(target: &FloatPoint, x: &f64, amount: &f64) -> FloatPoint {
   let stretch = (amount * PI).cos();

   FloatPoint::new(
      (target.x - x) * stretch + x,
      target.y,
   )
}
wrap_3_arg!(flip_x_fpt_f64_f64, flip_x_fpt_f64_f64_);


fn flip_y_fpt_f64_f64_(target: &FloatPoint, y: &f64, amount: &f64) -> FloatPoint {
   let stretch = (amount * PI).cos();

   FloatPoint::new(
      target.x,
      (target.y - y) * stretch + y,
   )
}
wrap_3_arg!(flip_y_fpt_f64_f64, flip_y_fpt_f64_f64_);


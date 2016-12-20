use super::value_ptr::ValuePtr;
use super::compiler::FnRef;
use super::execute::Executor;


fn equal_bln_bln_(a: &bool, b: &bool) -> bool {
   *a == *b
}
wrap_2_arg!(equal_bln_bln, equal_bln_bln_);


fn unequal_bln_bln_(a: &bool, b: &bool) -> bool {
   *a != *b
}
wrap_2_arg!(unequal_bln_bln, unequal_bln_bln_);

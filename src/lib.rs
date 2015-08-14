#![cfg_attr(target_os="linux", feature(cstr_to_str))]

extern crate libc;

pub mod error;
pub mod hook;
pub mod sys;

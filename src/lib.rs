#![cfg_attr(target_os="linux", feature(cstr_to_str))]

extern crate libc;

pub mod application;
pub mod window;
pub mod sys;
pub mod os;
pub mod error;
pub mod hook;

pub use application::Application;

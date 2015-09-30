#![cfg(target_os = "linux")]

pub mod ffi;
pub mod atom;
pub mod connection;
pub mod screen;
pub mod event;
pub mod window;

pub use self::connection::Connection;
pub use self::screen::Screen;
pub use self::window::Window;
pub use self::atom::Atom;
pub use self::event::{Event, EventType, EventIterator, ResizedEvent};

#[derive(PartialEq)]
pub struct XID {
   pub id: ffi::c_uint
}

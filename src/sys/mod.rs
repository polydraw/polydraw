pub mod utils;

pub mod dl;
pub mod x11;
pub mod xcb;
pub mod win32;
pub mod gl;
pub mod egl;
pub mod wgl;
pub mod glx;
pub mod cl;
pub mod ft;

#[cfg(target_os = "windows")]
pub use self::win32::WindowsDynLibrary as DynLibrary;
#[cfg(target_os = "linux")]
pub use self::dl::UnixDynLibrary as DynLibrary;

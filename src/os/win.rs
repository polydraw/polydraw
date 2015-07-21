#![cfg(target_os = "windows")]

pub mod ffi {
   #![allow(non_snake_case)]

   pub use libc::{
      c_int, c_uint, c_void, uintptr_t, c_ulong
   };
   pub use libc::types::os::arch::extra::{
      HANDLE, LONG_PTR, LRESULT, HINSTANCE, LPCWSTR, HMODULE
   };
   use std::mem;

   pub type HWND = HANDLE;
   pub type HICON = HANDLE;
   pub type HBRUSH = HANDLE;
   pub type HCURSOR = HICON;

   pub type WPARAM = uintptr_t;
   pub type LPARAM = LONG_PTR;

   pub type ATOM = u16;

   pub type WNDPROC = Option<unsafe extern "system" fn(HWND, c_uint, WPARAM, LPARAM) -> LRESULT>;

   pub const CS_VREDRAW:              c_ulong = 0x0001;
   pub const CS_HREDRAW:              c_ulong = 0x0002;
   pub const CS_DBLCLKS:              c_ulong = 0x0008;
   pub const CS_OWNDC:                c_ulong = 0x0020;
   pub const CS_CLASSDC:              c_ulong = 0x0040;
   pub const CS_PARENTDC:             c_ulong = 0x0080;
   pub const CS_NOCLOSE:              c_ulong = 0x0200;
   pub const CS_SAVEBITS:             c_ulong = 0x0800;
   pub const CS_BYTEALIGNCLIENT:      c_ulong = 0x1000;
   pub const CS_BYTEALIGNWINDOW:      c_ulong = 0x2000;
   pub const CS_GLOBALCLASS:          c_ulong = 0x4000;
   pub const CS_IME:                  c_ulong = 0x00010000;
   pub const CS_DROPSHADOW:           c_ulong = 0x00020000;

   #[repr(C)]
   #[derive(Copy)]
   pub struct WNDCLASSEXW {
      pub cbSize: c_uint,
      pub style: c_uint,
      pub lpfnWndProc: WNDPROC,
      pub cbClsExtra: c_int,
      pub cbWndExtra: c_int,
      pub hInstance: HINSTANCE,
      pub hIcon: HICON,
      pub hCursor: HCURSOR,
      pub hbrBackground: HBRUSH,
      pub lpszMenuName: LPCWSTR,
      pub lpszClassName: LPCWSTR,
      pub hIconSm: HICON,
   }
   impl Clone for WNDCLASSEXW {
      fn clone(&self) -> Self { *self }
   }
   impl Default for WNDCLASSEXW {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   extern "system" {
      pub fn DefWindowProcW(hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
      pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
      pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;
   }
}

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;


fn to_utf16_os<S: AsRef<OsStr> + ?Sized>(s: &S) -> Vec<u16> {
   let mut v: Vec<_> = OsStr::new(s).encode_wide().collect();
   v.push(0);
   v
}

pub fn register_window_class<S: AsRef<OsStr> + ?Sized>(name: &S) {
   unsafe extern "system" fn wnd_proc(
      hwnd: ffi::HWND,
      msg: ffi::c_uint,
      wparam: ffi::WPARAM,
      lparam: ffi::LPARAM
   ) -> ffi::LRESULT {
      ffi::DefWindowProcW(hwnd, msg, wparam, lparam)
   }

   unsafe {
      let wnd_class = ffi::WNDCLASSEXW {
         cbSize: mem::size_of::<ffi::WNDCLASSEXW>() as ffi::c_uint,
         style: ffi::CS_HREDRAW | ffi::CS_VREDRAW | ffi::CS_OWNDC,
         lpfnWndProc: Some(wnd_proc),
         cbClsExtra: 0,
         cbWndExtra: 0,
         hInstance: ffi::GetModuleHandleW(ptr::null()),
         hIcon: ptr::null_mut(),
         hCursor: ptr::null_mut(),
         hbrBackground: ptr::null_mut(),
         lpszMenuName: ptr::null(),
         lpszClassName: to_utf16_os(name).as_ptr(),
         hIconSm: ptr::null_mut(),
      };

      ffi::RegisterClassExW(&wnd_class);
   }
}

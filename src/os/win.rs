#![cfg(target_os = "windows")]

pub mod ffi {
   #![allow(non_snake_case)]

   pub use libc::{
      c_int, c_uint, c_void, uintptr_t, c_long, c_ulong
   };
   pub use libc::types::os::arch::extra::{
      HANDLE, LONG_PTR, LRESULT, HINSTANCE, LPCWSTR, HMODULE, LPVOID, BOOL
   };
   use std::mem;

   pub type HWND = HANDLE;
   pub type HICON = HANDLE;
   pub type HBRUSH = HANDLE;
   pub type HMENU = HANDLE;
   pub type HDC = HANDLE;
   pub type HCURSOR = HICON;

   pub type WPARAM = uintptr_t;
   pub type LPARAM = LONG_PTR;

   pub type ATOM = u16;

   pub type WNDPROC = Option<unsafe extern "system" fn(HWND, c_uint, WPARAM, LPARAM) -> LRESULT>;

   pub const CS_VREDRAW:                 c_ulong = 0x00000001;
   pub const CS_HREDRAW:                 c_ulong = 0x00000002;
   pub const CS_DBLCLKS:                 c_ulong = 0x00000008;
   pub const CS_OWNDC:                   c_ulong = 0x00000020;
   pub const CS_CLASSDC:                 c_ulong = 0x00000040;
   pub const CS_PARENTDC:                c_ulong = 0x00000080;
   pub const CS_NOCLOSE:                 c_ulong = 0x00000200;
   pub const CS_SAVEBITS:                c_ulong = 0x00000800;
   pub const CS_BYTEALIGNCLIENT:         c_ulong = 0x00001000;
   pub const CS_BYTEALIGNWINDOW:         c_ulong = 0x00002000;
   pub const CS_GLOBALCLASS:             c_ulong = 0x00004000;
   pub const CS_IME:                     c_ulong = 0x00010000;
   pub const CS_DROPSHADOW:              c_ulong = 0x00020000;

   pub const WS_OVERLAPPED:              c_ulong = 0x00000000;
   pub const WS_POPUP:                   c_ulong = 0x80000000;
   pub const WS_CHILD:                   c_ulong = 0x40000000;
   pub const WS_MINIMIZE:                c_ulong = 0x20000000;
   pub const WS_VISIBLE:                 c_ulong = 0x10000000;
   pub const WS_DISABLED:                c_ulong = 0x08000000;
   pub const WS_CLIPSIBLINGS:            c_ulong = 0x04000000;
   pub const WS_CLIPCHILDREN:            c_ulong = 0x02000000;
   pub const WS_MAXIMIZE:                c_ulong = 0x01000000;
   pub const WS_CAPTION:                 c_ulong = 0x00C00000;
   pub const WS_BORDER:                  c_ulong = 0x00800000;
   pub const WS_DLGFRAME:                c_ulong = 0x00400000;
   pub const WS_VSCROLL:                 c_ulong = 0x00200000;
   pub const WS_HSCROLL:                 c_ulong = 0x00100000;
   pub const WS_SYSMENU:                 c_ulong = 0x00080000;
   pub const WS_THICKFRAME:              c_ulong = 0x00040000;
   pub const WS_GROUP:                   c_ulong = 0x00020000;
   pub const WS_TABSTOP:                 c_ulong = 0x00010000;
   pub const WS_MINIMIZEBOX:             c_ulong = 0x00020000;
   pub const WS_MAXIMIZEBOX:             c_ulong = 0x00010000;
   pub const WS_TILED:                   c_ulong = WS_OVERLAPPED;
   pub const WS_ICONIC:                  c_ulong = WS_MINIMIZE;
   pub const WS_SIZEBOX:                 c_ulong = WS_THICKFRAME;
   pub const WS_TILEDWINDOW:             c_ulong = WS_OVERLAPPEDWINDOW;
   pub const WS_OVERLAPPEDWINDOW:        c_ulong = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
   pub const WS_POPUPWINDOW:             c_ulong = WS_POPUP | WS_BORDER | WS_SYSMENU;
   pub const WS_CHILDWINDOW:             c_ulong = WS_CHILD;

   pub const WS_EX_DLGMODALFRAME:        c_ulong = 0x00000001;
   pub const WS_EX_NOPARENTNOTIFY:       c_ulong = 0x00000004;
   pub const WS_EX_TOPMOST:              c_ulong = 0x00000008;
   pub const WS_EX_ACCEPTFILES:          c_ulong = 0x00000010;
   pub const WS_EX_TRANSPARENT:          c_ulong = 0x00000020;
   pub const WS_EX_MDICHILD:             c_ulong = 0x00000040;
   pub const WS_EX_TOOLWINDOW:           c_ulong = 0x00000080;
   pub const WS_EX_WINDOWEDGE:           c_ulong = 0x00000100;
   pub const WS_EX_CLIENTEDGE:           c_ulong = 0x00000200;
   pub const WS_EX_CONTEXTHELP:          c_ulong = 0x00000400;
   pub const WS_EX_RIGHT:                c_ulong = 0x00001000;
   pub const WS_EX_LEFT:                 c_ulong = 0x00000000;
   pub const WS_EX_RTLREADING:           c_ulong = 0x00002000;
   pub const WS_EX_LTRREADING:           c_ulong = 0x00000000;
   pub const WS_EX_LEFTSCROLLBAR:        c_ulong = 0x00004000;
   pub const WS_EX_RIGHTSCROLLBAR:       c_ulong = 0x00000000;
   pub const WS_EX_CONTROLPARENT:        c_ulong = 0x00010000;
   pub const WS_EX_STATICEDGE:           c_ulong = 0x00020000;
   pub const WS_EX_APPWINDOW:            c_ulong = 0x00040000;
   pub const WS_EX_OVERLAPPEDWINDOW:     c_ulong = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
   pub const WS_EX_PALETTEWINDOW:        c_ulong = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
   pub const WS_EX_LAYERED:              c_ulong = 0x00080000;
   pub const WS_EX_NOINHERITLAYOUT:      c_ulong = 0x00100000;
   pub const WS_EX_NOREDIRECTIONBITMAP:  c_ulong = 0x00200000;
   pub const WS_EX_LAYOUTRTL:            c_ulong = 0x00400000;
   pub const WS_EX_COMPOSITED:           c_ulong = 0x02000000;
   pub const WS_EX_NOACTIVATE:           c_ulong = 0x08000000;

   pub const CW_USEDEFAULT:                c_int = 0x80000000u32 as c_int;

   pub const WM_CLOSE:                    c_uint = 16;

   pub const SW_SHOWNORMAL:                c_int = 1;
   pub const SW_SHOWMINIMIZED:             c_int = 2;
   pub const SW_MAXIMIZE:                  c_int = 3;
   pub const SW_SHOWMAXIMIZED:             c_int = 3;
   pub const SW_SHOWNOACTIVATE:            c_int = 4;
   pub const SW_SHOW:                      c_int = 5;
   pub const SW_MINIMIZE:                  c_int = 6;
   pub const SW_SHOWMINNOACTIVE:           c_int = 7;
   pub const SW_SHOWNA:                    c_int = 8;
   pub const SW_RESTORE:                   c_int = 9;
   pub const SW_SHOWDEFAULT:               c_int = 10;
   pub const SW_FORCEMINIMIZE:             c_int = 11;

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

   #[repr(C)]
   #[derive(Copy)]
   pub struct POINT {
      pub x: c_long,
      pub y: c_long,
   }
   impl Clone for POINT {
      fn clone(&self) -> Self { *self }
   }
   impl Default for POINT {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   #[repr(C)]
   #[derive(Copy)]
   pub struct MSG {
      pub hwnd: HWND,
      pub message: c_uint,
      pub wParam: WPARAM,
      pub lParam: LPARAM,
      pub time: c_ulong,
      pub pt: POINT,
   }
   impl Clone for MSG {
      fn clone(&self) -> Self { *self }
   }
   impl Default for MSG {
      fn default() -> Self { unsafe { mem::zeroed() } }
   }

   extern "system" {
      pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

      pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;

      pub fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;

      pub fn PostQuitMessage(nExitCode: c_int);

      pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;

      pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;

      pub fn GetDC(hwnd: HWND) -> HDC;

      pub fn DefWindowProcW(
         hwnd: HWND,
         Msg: c_uint,
         wParam: WPARAM,
         lParam: LPARAM
      ) -> LRESULT;

      pub fn CreateWindowExW(
         dwExStyle: c_ulong,
         lpClassName: LPCWSTR,
         lpWindowName: LPCWSTR,
         dwStyle: c_ulong,
         x: c_int,
         y: c_int,
         nWidth: c_int,
         nHeight: c_int,
         hWndParent: HWND,
         hMenu: HMENU,
         hInstance: HINSTANCE,
         lpParam: LPVOID,
      ) -> HWND;

      pub fn GetMessageW(
         lpMsg: *const MSG,
         hwnd: HWND,
         wMsgFilterMin: c_uint,
         wMsgFilterMax: c_uint
      ) -> BOOL;
   }
}

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;


fn to_utf16_os(s: &str) -> Vec<u16> {
   let mut v: Vec<_> = OsStr::new(s).encode_wide().collect();
   v.push(0);
   v
}

pub fn register_window_class(class_name: &str) {
   unsafe extern "system" fn wnd_proc(
      hwnd: ffi::HWND,
      msg: ffi::c_uint,
      wparam: ffi::WPARAM,
      lparam: ffi::LPARAM
   ) -> ffi::LRESULT {
      match msg {
         ffi::WM_CLOSE => {
            ffi::PostQuitMessage(0);
         },
         _ => {
            return ffi::DefWindowProcW(hwnd, msg, wparam, lparam);
         }
      }

      return 0;
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
         lpszClassName: to_utf16_os(class_name).as_ptr(),
         hIconSm: ptr::null_mut(),
      };

      ffi::RegisterClassExW(&wnd_class);
   }
}

pub struct Window {
   pub hwnd: ffi::HWND
}

impl Window {
   pub fn create(width: ffi::c_int, height: ffi::c_int, title: &str, class_name: &str) -> Self {
      let hwnd = unsafe {
         ffi::CreateWindowExW(
            ffi::WS_EX_CLIENTEDGE,
            to_utf16_os(class_name).as_ptr(),
            to_utf16_os(title).as_ptr(),
            ffi::WS_OVERLAPPEDWINDOW | ffi::WS_VISIBLE,
            ffi::CW_USEDEFAULT, ffi::CW_USEDEFAULT,
            width, height,
            ptr::null_mut(),
            ptr::null_mut(),
            ffi::GetModuleHandleW(ptr::null()),
            ptr::null_mut()
         )
      };

      Window {
         hwnd: hwnd
      }
   }

   pub fn dc(&self) -> ffi::HDC {
      unsafe {
         ffi::GetDC(self.hwnd)
      }
   }

   pub fn show_normal(&self) {
      unsafe {
         ffi::ShowWindow(self.hwnd, ffi::SW_SHOWNORMAL)
      };
   }
}

pub struct Message {
   msg: ffi::MSG
}

impl Message {
   pub fn get() -> Option<Self> {
      let mut msg = unsafe { mem::uninitialized() };

      match unsafe { ffi::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) } {
         0 => None,
         _ => Some(Message {
            msg: msg
         })
      }
   }

   pub fn translate(&self) {
      unsafe {
         ffi::TranslateMessage(&self.msg)
      };
   }

   pub fn dispatch(&self) {
      unsafe {
         ffi::DispatchMessageW(&self.msg)
      };
   }
}

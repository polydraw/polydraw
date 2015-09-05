#![cfg(target_os = "windows")]

pub mod ffi;

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn to_utf16_os(s: &str) -> Vec<u16> {
   let mut v: Vec<_> = OsStr::new(s).encode_wide().collect();
   v.push(0);
   v
}

#[allow(unused_variables)]
unsafe extern "system" fn wnd_proc(
   hwnd: ffi::HWND,
   msg: ffi::c_uint,
   wparam: ffi::WPARAM,
   lparam: ffi::LPARAM
) -> ffi::LRESULT {

   let window_ptr = ffi::GetWindowLongPtrW(hwnd, ffi::GWLP_USERDATA) as *mut Window;

   match msg {
      ffi::WM_NCCREATE => {
         let create_struct: *const ffi::CREATESTRUCTW = lparam as *const _;
         ffi::SetWindowLongPtrW(hwnd, ffi::GWLP_USERDATA, (*create_struct).lpCreateParams as ffi::LONG_PTR);
      },
      ffi::WM_CLOSE => {
         ffi::PostQuitMessage(0);
         return 0;
      },
      ffi::WM_CREATE => {
         return 0;
      },
      _ => {
         return ffi::DefWindowProcW(hwnd, msg, wparam, lparam);
      }
   }

   ffi::DefWindowProcW(hwnd, msg, wparam, lparam)
}

pub struct Window {
   pub hwnd: ffi::HWND
}

impl Window {
   pub fn new(width: u32, height: u32, title: &str, class_name: &str) -> Self {
      Self::register_class(class_name);

      let mut window: Window = Window {
         hwnd: ptr::null_mut(),
      };

      let window_ptr = &mut window as *mut Window as *mut ffi::c_void;

      let hwnd = unsafe {
         ffi::CreateWindowExW(
            ffi::WS_EX_APPWINDOW | ffi::WS_EX_WINDOWEDGE,
            to_utf16_os(class_name).as_ptr(),
            to_utf16_os(title).as_ptr(),
            ffi::WS_OVERLAPPEDWINDOW | ffi::WS_CLIPSIBLINGS | ffi::WS_CLIPCHILDREN,
            ffi::CW_USEDEFAULT, ffi::CW_USEDEFAULT,
            width as ffi::c_int, height as ffi::c_int,
            ptr::null_mut(),
            ptr::null_mut(),
            ffi::GetModuleHandleW(ptr::null()),
            window_ptr
         )
      };

      window.hwnd = hwnd;

      window
   }

   pub fn register_class(class_name: &str) {
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

   pub fn device_context(&self) -> DeviceContext {
      let hdc = unsafe {
         ffi::GetDC(self.hwnd)
      };

      DeviceContext {
         hdc: hdc
      }
   }

   pub fn show_normal(&self) {
      unsafe {
         ffi::ShowWindow(self.hwnd, ffi::SW_SHOWNORMAL)
      };
   }
}

pub struct DeviceContext {
   pub hdc: ffi::HDC
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

   pub fn peek() -> Option<Self> {
      let mut msg = unsafe { mem::uninitialized() };

      match unsafe { ffi::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, ffi::PM_REMOVE) } {
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

   pub fn is_quit(&self) -> bool {
      return self.msg.message == ffi::WM_QUIT
   }
}

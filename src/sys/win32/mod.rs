#![cfg(target_os = "windows")]

pub mod ffi;
use std::ffi::CString;

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use error::{RuntimeError, ErrorKind};

use super::utils::fn_ptr::{FnPtrLoader, FnPtr};

fn to_utf16_os(s: &str) -> Vec<u16> {
   let mut v: Vec<_> = OsStr::new(s).encode_wide().collect();
   v.push(0);
   v
}

pub struct Window {
   pub hwnd: ffi::HWND
}

impl Window {
   pub fn new(
      width: u32,
      height: u32,
      title: &str,
      class_name: &str,
      wnd_proc: ffi::WNDPROC
   ) -> Self {

      Self::register_class(class_name, wnd_proc);

      let mut rect = ffi::RECT {
         left: 0,
         top: 0,
         right: width as ffi::c_int,
         bottom: height as ffi::c_int,
      };

      let ex_style = ffi::WS_EX_APPWINDOW | ffi::WS_EX_WINDOWEDGE;

      let style = ffi::WS_OVERLAPPEDWINDOW | ffi::WS_CLIPSIBLINGS | ffi::WS_CLIPCHILDREN;

      unsafe {
         ffi::AdjustWindowRectEx(&mut rect, style, 0, ex_style)
      };

      let width = rect.right - rect.left;
      let height = rect.bottom - rect.top;

      let hwnd = unsafe {
         ffi::CreateWindowExW(
            ex_style,
            to_utf16_os(class_name).as_ptr(),
            to_utf16_os(title).as_ptr(),
            style,
            ffi::CW_USEDEFAULT, ffi::CW_USEDEFAULT,
            width as ffi::c_int, height as ffi::c_int,
            ptr::null_mut(),
            ptr::null_mut(),
            ffi::GetModuleHandleW(ptr::null()),
            ptr::null_mut()
         )
      };

      Window {
         hwnd: hwnd,
      }
   }

   #[inline]
   pub fn register_class(class_name: &str, wnd_proc: ffi::WNDPROC) {
      unsafe {
         let wnd_class = ffi::WNDCLASSEXW {
            cbSize: mem::size_of::<ffi::WNDCLASSEXW>() as ffi::c_uint,
            style: ffi::CS_HREDRAW | ffi::CS_VREDRAW | ffi::CS_OWNDC,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: ffi::GetModuleHandleW(ptr::null()),
            hIcon: ptr::null_mut(),
            hCursor: Cursor::default().hcursor,
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: to_utf16_os(class_name).as_ptr(),
            hIconSm: ptr::null_mut(),
         };

         ffi::RegisterClassExW(&wnd_class);
      }
   }

   #[inline]
   pub fn device_context(&self) -> DeviceContext {
      let hdc = unsafe {
         ffi::GetDC(self.hwnd)
      };

      DeviceContext {
         hdc: hdc
      }
   }

   #[inline]
   pub fn show_normal(&self) {
      unsafe {
         ffi::ShowWindow(self.hwnd, ffi::SW_SHOWNORMAL)
      };
   }

   #[inline]
   pub fn position(&self, x: i32, y: i32) {
      unsafe {
         ffi::SetWindowPos(
            self.hwnd,
            ptr::null_mut(),
            x as ffi::c_int, y as ffi::c_int,
            0, 0,
            ffi::SWP_NOZORDER | ffi::SWP_NOSIZE
         )
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

   #[inline]
   pub fn peek() -> Option<Self> {
      let mut msg = unsafe { mem::uninitialized() };

      match unsafe { ffi::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, ffi::PM_REMOVE) } {
         0 => None,
         _ => Some(Message {
            msg: msg
         })
      }
   }

   #[inline]
   pub fn translate(&self) {
      unsafe {
         ffi::TranslateMessage(&self.msg)
      };
   }

   #[inline]
   pub fn dispatch(&self) {
      unsafe {
         ffi::DispatchMessageW(&self.msg)
      };
   }

   #[inline]
   pub fn is_quit(&self) -> bool {
      return self.msg.message == ffi::WM_QUIT
   }
}

pub struct DeviceContext {
   pub hdc: ffi::HDC
}

pub struct DeviceMode {
   pub device_mode: ffi::DEVMODEW
}

impl DeviceMode {
   pub fn enumerate() -> Self {
      let mut device_mode = ffi::DEVMODEW::default();
      device_mode.dmSize = mem::size_of::<ffi::DEVMODEW>() as ffi::WORD;

      unsafe {
         ffi::EnumDisplaySettingsW(
            ptr::null(),
            ffi::ENUM_CURRENT_SETTINGS,
            &mut device_mode
         )
      };

      DeviceMode {
         device_mode: device_mode,
      }
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      (self.device_mode.dmPelsWidth, self.device_mode.dmPelsHeight)
   }
}

pub struct Cursor {
   pub hcursor: ffi::HCURSOR
}

impl Cursor {
   #[inline]
   pub fn activate(&self) {
      unsafe {
         ffi::SetCursor(self.hcursor)
      };
   }
}

impl Default for Cursor {
   fn default() -> Self {
      let hcursor = unsafe {
         ffi::LoadCursorW(
            ptr::null_mut(),
            ffi::IDC_ARROW
         )
      };

      Cursor {
         hcursor: hcursor
      }
   }
}

pub struct Library {
   pub handle: ffi::HMODULE
}

impl Library {
   pub fn new(name: &str) -> Result<Self, RuntimeError> {
      let handle = unsafe {
         ffi::LoadLibraryW(to_utf16_os(name).as_ptr())
      };

      if handle.is_null() {
         return Err(RuntimeError::new(
            ErrorKind::Win32,
            format!("Loading dynamic library failed {}", name).to_string()
         ));
      }

      Ok(Library {
         handle: handle,
      })
   }
}

impl FnPtrLoader for Library {
   fn load(&self, name: &str) -> FnPtr {
      let cname = CString::new(name).unwrap();

      unsafe {
         ffi::GetProcAddress(self.handle, cname.as_ptr())
      }
   }
}

impl Drop for Library {
   fn drop(&mut self) {
      unsafe {
         ffi::FreeLibrary(self.handle)
      };
   }
}

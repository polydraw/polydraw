use sys::win32::ffi;

#[allow(unused_variables)]
pub unsafe extern "system" fn wnd_proc(
   hwnd: ffi::HWND,
   msg: ffi::c_uint,
   wparam: ffi::WPARAM,
   lparam: ffi::LPARAM
) -> ffi::LRESULT {
   match msg {
      ffi::WM_CLOSE => {
         ffi::PostQuitMessage(0);
         return 0;
      },
      ffi::WM_SIZE => {
         println!("WM SIZE");
      },
      ffi::WM_CREATE => {
         return 0;
      },
      _ => {}
   }

   ffi::DefWindowProcW(hwnd, msg, wparam, lparam)
}

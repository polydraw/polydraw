use std::sync::mpsc::Sender;
use std::cell::RefCell;

use sys::win32::ffi;

use event::Event;

thread_local!(
   pub static SENDER: RefCell<Option<Sender<Event>>> = RefCell::new(None)
);

fn send(event: Event) {
   SENDER.with(|sender_cell| {
      let sender_option = sender_cell.borrow();
      let sender = match *sender_option {
         Some(ref sender) => sender,
         None => return
      };

      sender.send(event).ok();
   });
}

#[allow(unused_variables)]
pub unsafe extern "system" fn wnd_proc(
   hwnd: ffi::HWND,
   msg: ffi::c_uint,
   wparam: ffi::WPARAM,
   lparam: ffi::LPARAM
) -> ffi::LRESULT {
   match msg {
      ffi::WM_CREATE => {
         0
      },

      ffi::WM_ERASEBKGND => {
         1
      },

      ffi::WM_SIZE => {
         let width = ffi::LOWORD(lparam as ffi::DWORD) as u32;
         let height = ffi::HIWORD(lparam as ffi::DWORD) as u32;
         send(Event::Resize(width, height));
         0
      },

      ffi::WM_CLOSE => {
         ffi::PostQuitMessage(0);
         send(Event::Quit);
         0
      },

      _ => {
         ffi::DefWindowProcW(hwnd, msg, wparam, lparam)
      }
   }
}

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
         send(Event::Resized(width, height));
         0
      },

      ffi::WM_MOUSEMOVE => {
         let x = ffi::GET_X_LPARAM(lparam);
         let y = ffi::GET_Y_LPARAM(lparam);
         send(Event::MouseMoved(x, y));
         0
      },

      ffi::WM_LBUTTONDOWN => {
         send(Event::MouseLeftButtonPressed);
         0
      },

      ffi::WM_LBUTTONUP => {
         send(Event::MouseLeftButtonReleased);
         0
      },

      ffi::WM_MBUTTONDOWN => {
         send(Event::MouseMiddleButtonPressed);
         0
      },

      ffi::WM_MBUTTONUP => {
         send(Event::MouseMiddleButtonReleased);
         0
      },

      ffi::WM_RBUTTONDOWN => {
         send(Event::MouseRightButtonPressed);
         0
      },

      ffi::WM_RBUTTONUP => {
         send(Event::MouseRightButtonReleased);
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

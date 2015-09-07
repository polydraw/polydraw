use std::thread;
use std::sync::mpsc::channel;
use std::str::FromStr;

use error::RuntimeError;

use sys::win32;

use super::wnd_proc::wnd_proc;

pub struct WindowsWindow {
   pub window: win32::Window,
   pub device_context: win32::DeviceContext,
}

unsafe impl Send for WindowsWindow {}
unsafe impl Sync for WindowsWindow {}

impl WindowsWindow {
   pub fn new(
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let title = String::from_str(title).unwrap();

      let (sender, receiver) = channel();

      thread::spawn(move || {
         sender.send(
            Self::create(&title, x, y, width, height)
         ).ok();

         Self::run_message_loop();
      });

      receiver.recv().unwrap()
   }

   fn create(
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(Self::init_window(&title, x, y, width, height));

      let device_context = window.device_context();

      Ok(WindowsWindow {
         window: window,
         device_context: device_context,
      })
   }

   #[inline]
   fn init_window(
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<win32::Window, RuntimeError> {

      let window = win32::Window::new(
         width, height, title, "PolyDrawWndClass", Some(wnd_proc)
      );

      window.show_normal();

      window.position(x, y);

      Ok(window)
   }

   fn run_message_loop() {
      loop {
         let message = match win32::Message::get() {
            Some(message) => message,
            None => break
         };

         if message.is_quit() {
            break
         }

         message.translate();
         message.dispatch();
      }
   }
}

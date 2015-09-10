use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::str::FromStr;

use error::RuntimeError;

use sys::win32;

use event::Event;

use super::wnd_proc::{wnd_proc, SENDER};

pub struct WindowsWindow {
   pub window: win32::Window,
   pub device_context: win32::DeviceContext,
   pub event_receiver: Receiver<Event>,
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

      let event_receiver = Self::init_event_receiver();

      Ok(WindowsWindow {
         window: window,
         device_context: device_context,
         event_receiver: event_receiver,
      })
   }

   #[inline]
   fn init_event_receiver() -> Receiver<Event> {
      let (sender, receiver) = channel();
      let mut sender = Some(sender);
      SENDER.with(|sender_cell| {
         (*sender_cell.borrow_mut()) = Some(sender.take().unwrap());
      });
      receiver
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

   #[inline]
   pub fn poll_events(&self) -> PollEventsIterator {
      PollEventsIterator {
         window: self,
      }
   }

   #[inline]
   pub fn wait_events(&self) -> WaitEventsIterator {
      WaitEventsIterator {
         window: self,
      }
   }
}

pub struct PollEventsIterator<'a> {
   window: &'a WindowsWindow,
}

impl<'a> Iterator for PollEventsIterator<'a> {
   type Item = Event;

   fn next(&mut self) -> Option<Event> {
      self.window.event_receiver.try_recv().ok()
   }
}

pub struct WaitEventsIterator<'a> {
   window: &'a WindowsWindow,
}

impl<'a> Iterator for WaitEventsIterator<'a> {
   type Item = Event;

   fn next(&mut self) -> Option<Event> {
     self.window.event_receiver.recv().ok()
   }
}

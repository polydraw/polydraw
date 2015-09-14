use std::rc::Rc;

use error::RuntimeError;

use sys::xcb;

use event::Event;

use super::display::LinuxDisplay;

pub struct XcbAtoms {
   pub protocols_atom: xcb::Atom,
   pub delete_window_atom: xcb::Atom,
}

pub struct LinuxWindow {
   pub window: xcb::Window,
   pub atoms: XcbAtoms,
}

impl LinuxWindow {
   pub fn new(
      display: &LinuxDisplay, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<Self, RuntimeError> {

      let window = try!(Self::init_window(
         &display.connection, &display.screen,
         title, x, y, width, height
      ));

      let atoms = try!(Self::init_atoms(&window));

      Ok(LinuxWindow {
         window: window,
         atoms: atoms,
      })
   }

   #[inline]
   pub fn init_window(
      connection: &Rc<xcb::Connection>, screen: &xcb::Screen,
      title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<xcb::Window, RuntimeError> {

      let window = try!(xcb::Window::create(
         connection, &screen, width, height,
      ));

      try!(window.set_title(title));

      try!(window.map());

      try!(window.position(x, y));

      Ok(window)
   }

   #[inline]
   pub fn init_atoms(window: &xcb::Window) -> Result<XcbAtoms, RuntimeError> {
      let (protocols_atom, delete_window_atom) = try!(window.register_close_event());

      Ok(XcbAtoms {
         protocols_atom: protocols_atom,
         delete_window_atom: delete_window_atom,
      })
   }

   #[inline]
   pub fn poll_events(&self) -> PollEventsIterator {
      PollEventsIterator::new(&self.window, &self.atoms)
   }
}

pub struct PollEventsIterator<'a> {
   xcb_iterator: xcb::EventIterator,
   atoms: &'a XcbAtoms,
}

impl<'a> PollEventsIterator<'a> {
   #[inline]
   pub fn new(window: &xcb::Window, atoms: &'a XcbAtoms) -> Self {
      PollEventsIterator {
         xcb_iterator: window.connection.poll_event_iter(),
         atoms: atoms
      }
   }

   #[inline]
   fn convert(&mut self, xcb_event: xcb::Event) -> Option<Event> {
      match xcb_event.event_type() {
         None => {},
         Some(event_type) => match event_type {
            xcb::EventType::ClientMessage => {
               if xcb_event.is_close_event(
                     &self.atoms.protocols_atom,
                     &self.atoms.delete_window_atom
               ) {
                  return Some(Event::Quit);
               }
            },

            xcb::EventType::ConfigureNotify => {
               let (_, width, height) = xcb_event.resize_properties();
               return Some(Event::Resized(width, height));
            },

            xcb::EventType::MotionNotify => {
               let (x, y) = xcb_event.mouse_move_properties();
               return Some(Event::MouseMoved(x, y));
            },

            _ => {}
         }
      }

      self.next()
   }
}

impl<'a> Iterator for PollEventsIterator<'a> {
   type Item = Event;

   #[inline]
   fn next(&mut self) -> Option<Event> {
      match self.xcb_iterator.next() {
         None => None,
         Some(result) => {
            match result {
               Err(e) => panic!(e.description),
               Ok(xcb_event) => {
                  self.convert(xcb_event)
               }
            }
         }
      }
   }
}

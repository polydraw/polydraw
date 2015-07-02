extern crate glx;

#[macro_use]
extern crate polydraw;

use std::ptr;

use polydraw::platform::x11::ffi;

fn screen_of_display(
   connection: *mut ffi::xcb_connection_t, screen: ffi::c_int
) -> *mut ffi::xcb_screen_t {

   let mut iter = unsafe {
      ffi::xcb_setup_roots_iterator(ffi::xcb_get_setup(connection))
   };

   let mut screen_num = screen;

   while screen_num > 0 && iter.rem != 0 {
      unsafe { ffi::xcb_screen_next(&mut iter) };
      screen_num -= 1;
   }

   iter.data
}


fn print_screen_info(screen: &ffi::xcb_screen_t) {
   println!("Informations of screen : {}", screen.root);
   println!("   width ............. : {}", screen.width_in_pixels);
   println!("   height ............ : {}", screen.height_in_pixels);
   println!("   white pixel ....... : {}", screen.white_pixel);
   println!("   black pixel ....... : {}", screen.black_pixel);
}

fn main() {
   let display = unsafe { ffi::XOpenDisplay(ptr::null()) };
   if display.is_null() {
      println!("Can't open display");
      return;
   }

   let connection = unsafe { ffi::XGetXCBConnection(display) };
   if connection.is_null() {
      unsafe { ffi::XCloseDisplay(display) };
      println!("Can't get xcb connection from display");
      return;
   }

   unsafe {
      ffi::XSetEventQueueOwner(display, ffi::XCBOwnsEventQueue)
   };

   let default_screen = DefaultScreen!(display);

   let screen = screen_of_display(connection, default_screen);

   unsafe { print_screen_info(&(*screen)) };

   let mut nelements = 0;

   let fb_configs = unsafe {
      glx::GetFBConfigs(
         display as *mut glx::types::Display,
         default_screen,
         &mut nelements
      )
   };

   println!("nelements ............ : {}", nelements);

   let mut visual_id = 0;

   unsafe {
      glx::GetFBConfigAttrib(
         display as *mut glx::types::Display,
         *fb_configs,
         glx::VISUAL_ID as i32,
         &mut visual_id
      )
   };

   println!("visual_id ............ : {}", visual_id);

   let context = unsafe {
      glx::CreateNewContext(
         display as *mut glx::types::Display,
         *fb_configs,
         glx::RGBA_TYPE as i32,
         ptr::null(),
         1
      )
   };

   println!("GL context ........... : {:?}", context);

   let colormap = unsafe { ffi::xcb_generate_id(connection) };
   let window = unsafe { ffi::xcb_generate_id(connection) };

   println!("colormap ............. : {:?}", colormap);
   println!("window ............... : {:?}", window);

   let colormap_res = unsafe {
      ffi::xcb_create_colormap(
         connection,
         ffi::XCB_COLORMAP_ALLOC_NONE as u8,
         colormap,
         (*screen).root,
         visual_id as u32
      )
   };

   println!("colormap res ......... : {:?}", colormap_res.sequence);

   let eventmask = ffi::XCB_EVENT_MASK_EXPOSURE | ffi::XCB_EVENT_MASK_KEY_PRESS;
   let valuelist = [eventmask, colormap, 0];
   let valuemask = ffi::XCB_CW_EVENT_MASK | ffi::XCB_CW_COLORMAP;

   let window_res = unsafe {
      ffi::xcb_create_window(
         connection,
         ffi::XCB_COPY_FROM_PARENT as u8,
         window,
         (*screen).root,
         0, 0,
         150, 150,
         0,
         ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
         visual_id as u32,
         valuemask,
         valuelist.as_ptr()
      )
   };

   println!("window res ........... : {:?}", window_res.sequence);

   let map_res = unsafe {
      ffi::xcb_map_window(connection, window)
   };

   println!("map res .............. : {:?}", map_res.sequence);

   let glxwindow = unsafe {
      glx::CreateWindow(
         display as *mut glx::types::Display,
         *fb_configs,
         window as u64,
         ptr::null()
      )
   };

   println!("glxwindow ............ : {:?}", glxwindow);

   let drawable = glxwindow;

   let made_current = unsafe {
      glx::MakeContextCurrent(
         display as *mut glx::types::Display,
         drawable,
         drawable,
         context
      )
   };

   println!("made current ......... : {:?}", made_current);

   unsafe {
      glx::DestroyWindow(
         display as *mut glx::types::Display,
         glxwindow
      )
   };

   unsafe {
      ffi::xcb_destroy_window(connection, window)
   };

   unsafe {
      glx::DestroyContext(display as *mut glx::types::Display, context)
   };

   unsafe { ffi::XCloseDisplay(display) };
}

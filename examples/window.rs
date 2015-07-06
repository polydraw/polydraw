#[macro_use]
extern crate polydraw;

use std::mem;

use polydraw::platform::x11::ffi;
use polydraw::os::x11::{Display};

fn screen_of_display(
   connection: *mut ffi::xcb_connection_t,
   screen: ffi::c_int
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
   let display = match Display::default() {
      Ok(display) => display,
      Err(e) => {
         panic!(e.description);
      }
   };

   let ptr = display.display_ptr as *mut ffi::Display;

   let conn = match display.xcb_connection() {
      Ok(conn) => conn,
      Err(e) => {
         panic!(e.description);
      }
   };

   let connection = conn.connection_ptr as *mut ffi::xcb_connection_t;

   display.xcb_own_event_queue();

   let default_screen = DefaultScreen!(ptr);

   let screen = screen_of_display(connection, default_screen);

   unsafe { print_screen_info(&(*screen)) };

   let window = unsafe { ffi::xcb_generate_id(connection) };

   println!("window ............... : {:?}", window);

   let eventmask = ffi::XCB_EVENT_MASK_EXPOSURE |
      ffi::XCB_EVENT_MASK_KEY_PRESS;
   let valuelist = [eventmask, 0];
   let valuemask = ffi::XCB_CW_EVENT_MASK;

   let window_res = unsafe {
      ffi::xcb_create_window(
         connection,
         ffi::XCB_COPY_FROM_PARENT as u8,
         window,
         (*screen).root,
         0, 0,
         800, 450,
         0,
         ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
         (*screen).root_visual,
         valuemask,
         valuelist.as_ptr()
      )
   };

   println!("window res ........... : {:?}", window_res.sequence);

   let map_res = unsafe {
      ffi::xcb_map_window(connection, window)
   };

   println!("map res .............. : {:?}", map_res.sequence);

   if unsafe { ffi::eglBindAPI(ffi::EGL_OPENGL_API) } == 0 {
      panic!("eglBindAPI failed");
   }

   let egl_display = unsafe { ffi::eglGetDisplay(ptr) };

   println!("egl display .......... : {:?}", egl_display);

   let mut major: ffi::EGLint = unsafe { mem::uninitialized() };
   let mut minor: ffi::EGLint = unsafe { mem::uninitialized() };

   if unsafe { ffi::eglInitialize(egl_display, &mut major, &mut minor) } == 0 {
      panic!("eglInitialize failed");
   }

   println!("egl version .......... : {:?}.{:?}", major, minor);

   let config_attribs = [
      ffi::EGL_COLOR_BUFFER_TYPE,    ffi::EGL_RGB_BUFFER,
      ffi::EGL_BUFFER_SIZE,          32,
      ffi::EGL_RED_SIZE,             8,
      ffi::EGL_GREEN_SIZE,           8,
      ffi::EGL_BLUE_SIZE,            8,
      ffi::EGL_ALPHA_SIZE,           8,

      ffi::EGL_DEPTH_SIZE,           24,
      ffi::EGL_STENCIL_SIZE,         8,

      ffi::EGL_SAMPLE_BUFFERS,       0,
      ffi::EGL_SAMPLES,              0,

      ffi::EGL_SURFACE_TYPE,         ffi::EGL_WINDOW_BIT,
      ffi::EGL_RENDERABLE_TYPE,      ffi::EGL_OPENGL_BIT,

      ffi::EGL_NONE
   ];

   let mut num_config: ffi::EGLint = unsafe { mem::uninitialized() };
   let mut configs: [ffi::EGLConfig; 64] = unsafe { mem::uninitialized() };

   let chosen = unsafe {
      ffi::eglChooseConfig(
         egl_display,
         config_attribs.as_ptr() as *const _,
         configs.as_mut_ptr() as *mut *mut _,
         64,
         &mut num_config
      )
   };
   if chosen == 0 {
      panic!("eglChooseConfig failed");
   }

   println!("num config ........... : {:?}", num_config);

   if num_config == 0 {
      panic!("Failed to find suitable EGLConfig");
   }

   let config = configs[0];

   let context_attribs = [ffi::EGL_NONE];

   let context = unsafe {
      ffi::eglCreateContext(
         egl_display,
         config as *mut _,
         ffi::EGL_NO_CONTEXT as *mut _,
         context_attribs.as_ptr() as *const _,
      )
   };
   if context.is_null() {
      panic!("eglCreateContext failed");
   }

   let surface_attribs = [
      ffi::EGL_RENDER_BUFFER, ffi::EGL_BACK_BUFFER,
      ffi::EGL_NONE
   ];

   let surface = unsafe {
      ffi::eglCreateWindowSurface(
         egl_display,
         config as *mut _,
         window,
         surface_attribs.as_ptr() as *const _,
      )
   };
   if surface.is_null() {
      panic!("eglCreateWindowSurface failed");
   }

   let made_current = unsafe {
      ffi::eglMakeCurrent(egl_display, surface, surface, context)
   };
   if made_current == 0 {
      panic!("eglMakeCurrent failed");
   }

   let mut render_buffer: ffi::EGLint = unsafe { mem::uninitialized() };

   let ok = unsafe {
      ffi::eglQueryContext(
         egl_display,
         context,
         ffi::EGL_RENDER_BUFFER as i32,
         &mut render_buffer
      )
   };

   if ok == 0 {
      panic!("eglQueyContext(EGL_RENDER_BUFFER) failed");
   }

   if render_buffer == ffi::EGL_SINGLE_BUFFER as i32 {
      println!("warn: EGL surface is single buffered");
   }

   loop {
      let event = unsafe {
         ffi::xcb_wait_for_event(connection)
      };
      if event.is_null() {
         break;
      }

      let event_type = unsafe { (*event).response_type & !0x80 };

      match event_type {
         ffi::XCB_KEY_PRESS => {
            unsafe { ffi::free(event as *mut ffi::c_void) };
            break;
         },
         ffi::XCB_EXPOSE => {
            unsafe {
               ffi::glClearColor(0.0, 0.7, 1.0, 1.0);
               ffi::glClear(ffi::GL_COLOR_BUFFER_BIT);
               ffi::glFlush();

               ffi::eglSwapBuffers(egl_display, surface);
            };
         }
         _ => {}
      }

      unsafe { ffi::free(event as *mut ffi::c_void) };
   }

   unsafe {
      ffi::xcb_destroy_window(connection, window)
   };
}

extern crate gleam;

#[macro_use]
extern crate polydraw;

use std::ptr;
use std::mem;
use std::ffi::CString;

use gleam::gl;
use polydraw::platform::x11::ffi;
use polydraw::platform::egl;

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
   let display = unsafe { ffi::XOpenDisplay(ptr::null()) };
   if display.is_null() {
      panic!("Can't open display");
   }

   let connection = unsafe { ffi::XGetXCBConnection(display) };
   if connection.is_null() {
      unsafe { ffi::XCloseDisplay(display) };
      panic!("Can't get xcb connection from display");
   }

   unsafe {
      ffi::XSetEventQueueOwner(display, ffi::XCBOwnsEventQueue)
   };

   let default_screen = DefaultScreen!(display);

   let screen = screen_of_display(connection, default_screen);

   unsafe { print_screen_info(&(*screen)) };

   let window = unsafe { ffi::xcb_generate_id(connection) };

   println!("window ............... : {:?}", window);

   let eventmask = ffi::XCB_EVENT_MASK_EXPOSURE | ffi::XCB_EVENT_MASK_KEY_PRESS;
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

   let libegl = unsafe {
      ffi::dlopen(b"libEGL.so\0".as_ptr() as *const _, ffi::RTLD_NOW)
   };

   egl::load_with(|sym| {
      let sym = CString::new(sym).unwrap();
      unsafe { ffi::dlsym(libegl, sym.as_ptr()) }
   });

   if unsafe { egl::BindAPI(egl::OPENGL_API) } == 0 {
      panic!("eglBindAPI failed");
   }

   gl::load_with(|s| unsafe { egl::GetProcAddress(CString::new(s).unwrap().as_ptr() as *const _) as *const _ });

   let egl_display = unsafe { egl::GetDisplay(display) };

   println!("egl display .......... : {:?}", egl_display);

   let mut major: egl::types::EGLint = unsafe { mem::uninitialized() };
   let mut minor: egl::types::EGLint = unsafe { mem::uninitialized() };

   if unsafe { egl::Initialize(egl_display, &mut major, &mut minor) } == 0 {
       panic!("eglInitialize failed");
   }

   println!("egl version .......... : {:?}.{:?}", major, minor);

   let config_attribs = [
      egl::COLOR_BUFFER_TYPE,    egl::RGB_BUFFER,
      egl::BUFFER_SIZE,          32,
      egl::RED_SIZE,             8,
      egl::GREEN_SIZE,           8,
      egl::BLUE_SIZE,            8,
      egl::ALPHA_SIZE,           8,

      egl::DEPTH_SIZE,           24,
      egl::STENCIL_SIZE,         8,

      egl::SAMPLE_BUFFERS,       0,
      egl::SAMPLES,              0,

      egl::SURFACE_TYPE,         egl::WINDOW_BIT,
      egl::RENDERABLE_TYPE,      egl::OPENGL_BIT,

      egl::NONE
   ];

   let mut num_config: egl::types::EGLint = unsafe { mem::uninitialized() };
   let mut configs: [egl::types::EGLConfig; 64] = unsafe { mem::uninitialized() };

   let chosen = unsafe {
      egl::ChooseConfig(
         egl_display,
         config_attribs.as_ptr() as *const _,
         configs.as_mut_ptr(),
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

   let context_attribs = [egl::NONE];

   let context = unsafe {
      egl::CreateContext(
         egl_display,
         config,
         egl::NO_CONTEXT,
         context_attribs.as_ptr() as *const _,
      )
   };
   if context.is_null() {
      panic!("eglCreateContext failed");
   }

   let surface_attribs = [
      egl::RENDER_BUFFER, egl::BACK_BUFFER,
      egl::NONE
   ];

   let surface = unsafe {
      egl::CreateWindowSurface(
         egl_display,
         config,
         window,
         surface_attribs.as_ptr() as *const _,
      )
   };
   if surface.is_null() {
      panic!("eglCreateWindowSurface failed");
   }

   let made_current = unsafe {
      egl::MakeCurrent(egl_display, surface, surface, context)
   };
   if made_current == 0 {
      panic!("eglMakeCurrent failed");
   }

   let mut render_buffer: egl::types::EGLint = unsafe { mem::uninitialized() };

   let ok = unsafe {
      egl::QueryContext(
         egl_display,
         context,
         egl::RENDER_BUFFER as i32,
         &mut render_buffer
      )
   };

   if ok == 0 {
      panic!("eglQueyContext(EGL_RENDER_BUFFER) failed");
   }

   if render_buffer == egl::SINGLE_BUFFER as i32 {
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
               gl::clear_color(0.0, 0.7, 1.0, 1.0);
               gl::clear(gl::COLOR_BUFFER_BIT);
               gl::flush();

               egl::SwapBuffers(egl_display, surface);
            };
         }
         _ => {}
      }

      unsafe { ffi::free(event as *mut ffi::c_void) };
   }

   unsafe {
      ffi::xcb_destroy_window(connection, window)
   };

   unsafe { ffi::XCloseDisplay(display) };
}

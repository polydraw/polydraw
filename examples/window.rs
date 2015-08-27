#![cfg(target_os = "linux")]

extern crate polydraw;
extern crate time;

mod support;

use std::rc::Rc;

use polydraw::sys::xcb;
use polydraw::sys::x11;
use polydraw::sys::egl;
use polydraw::sys::gl;
use polydraw::sys::cl;

use support::{create_data, update_data};

fn print_screen_info(screen: &xcb::Screen) {
   println!("Informations of screen .... : {}", screen.root());
   println!("   width .................. : {}", screen.width_in_pixels());
   println!("   height ................. : {}", screen.height_in_pixels());
   println!("   white pixel ............ : {}", screen.white_pixel());
   println!("   black pixel ............ : {}", screen.black_pixel());
}

fn print_platforms_info(platforms: &Vec<cl::Platform>) {
   for (i, platform) in platforms.iter().enumerate() {
      println!("CL platform [{}] ........... : {:?}", i, platform.ptr);
      println!("   Name ................... : {}", platform.name().unwrap());
      println!("   Vendor ................. : {}", platform.vendor().unwrap());
      println!("   Profile ................ : {}", platform.profile().unwrap());
      println!("   Version ................ : {}", platform.version().unwrap());
      println!("   Extensions ............. : {}", platform.extensions().unwrap());

      let devices = match platform.all_devices() {
         Ok(devices) => devices,
         Err(e) => {
            panic!(e.description);
         }
      };

      for (j, device) in devices.iter().enumerate() {
         println!("   CL device [{}] .......... : {:?}", j, device.ptr);
         println!("      Name ................ : {}", device.name().unwrap());
         println!("      Vendor .............. : {}", device.vendor().unwrap());
         println!("      Profile ............. : {}", device.profile().unwrap());
         println!("      Version ............. : {}", device.version().unwrap());
         println!("      OpenCL C version .... : {}", device.opencl_c_version().unwrap());
         println!("      Driver version ...... : {}", device.driver_version().unwrap());
         println!("      Built-in kernels .... : {}", device.built_in_kernels().unwrap());
         println!("      Extensions .......... : {}", device.extensions().unwrap());
         println!("      Available ........... : {}", device.available().unwrap());
         println!("      Compiler available .. : {}", device.compiler_available().unwrap());
         println!("      Linker available .... : {}", device.linker_available().unwrap());
         println!("      Little endian ....... : {}", device.endian_little().unwrap());
         println!("      Error correction .... : {}", device.error_correction().unwrap());
         println!("      Unified memory ...... : {}", device.unified_memory().unwrap());
         println!("      Image support ....... : {}", device.image_support().unwrap());
      }
   }
}

#[allow(unused_assignments)]
fn main() {
   let mut width: usize = 1280;
   let mut height: usize = 720;

   let platforms = match cl::platforms() {
      Ok(platforms) => platforms,
      Err(e) => {
         panic!(e.description);
      }
   };

   print_platforms_info(&platforms);

   let display = match x11::Display::default() {
      Ok(display) => display,
      Err(e) => {
         panic!(e.description);
      }
   };

   display.xcb_own_event_queue();

   let connection = Rc::new(
      match display.xcb_connection() {
         Ok(connection) => connection,
         Err(e) => {
            panic!(e.description);
         }
      }
   );

   let screen_id = display.default_screen();

   let scr = connection.screen_of_display(&screen_id);

   print_screen_info(&scr);

   let window = match xcb::Window::create(
      &connection, &scr,
      0, 0, width as u16, height as u16,
   ) {
      Ok(window) => window,
      Err(e) => {
         panic!(e.description);
      }
   };

   window.map();

   let (protocols_atom, delete_window_atom) = window.register_close_event();

   match egl::bind_api(egl::API::OpenGL) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   let egl_d = match egl::Display::from_native(&display) {
      Ok(egl_d) => egl_d,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("egl display ............... : {:?}", egl_d.ptr);

   let version = match egl::initialize(&egl_d) {
      Ok(version) => version,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("egl version ............... : {:?}.{:?}", version.major, version.minor);

   let config = match egl::choose_config(&egl_d) {
      Ok(config) => config,
      Err(e) => {
         panic!(e.description);
      }
   };

   let context = match egl::create_context(&egl_d, &config) {
      Ok(context) => context,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("context ptr ............... : {:?}", context.ptr);

   gl::load(egl::Loader::new());

   let surface = match egl::create_window_surface(&egl_d, &config, &window.window_id.id) {
      Ok(surface) => surface,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("surface ptr ............... : {:?}", surface.ptr);

   match egl::make_current(&egl_d, &surface, &surface, &context) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   match egl::query_context(&egl_d, &context) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   gl::reset_pixelstore_alignment();

   let mut counter: u64 = 0;
   let mut data = create_data(width, height);

   let texture = gl::Texture::new(width, height);

   println!("GL texture ................ : {:?}", texture.name);

   let framebuffer = gl::Framebuffer::new(&texture);

   println!("GL framebuffer ............ : {:?}", framebuffer.name);

   match egl::swap_interval(&egl_d, 0) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   let start_time = time::precise_time_ns();

   let mut exit = false;
   let mut seed = 0;

   let mut new_width: usize = width;
   let mut new_height: usize = height;

   loop {
      for result in connection.poll_event_iter() {
         let event = match result {
            Ok(event) => event,
            Err(e) => {
               panic!(e.description);
            }
         };

         let event_type = event.event_type();

         match event_type {
            xcb::EventType::Expose | xcb::EventType::Empty => {
               counter += 1;
               seed = counter;

               if new_width != width || new_height != height {
                  width = new_width;
                  height = new_height;

                  data = create_data(width, height);
                  texture.resize(width, height);
               }

               update_data(&mut data, width, height, &mut seed);

               texture.update(width, height, &data);

               framebuffer.blit(width, height);

               gl::flush();

               match egl::swap_buffers(&egl_d, &surface) {
                  Ok(_) => {},
                  Err(e) => {
                     panic!(e.description);
                  }
               };
            },
            xcb::EventType::ClientMessage => {
               if event.is_close_event(&protocols_atom, &delete_window_atom) {
                  exit = true;
                  break;
               }
            },
            xcb::EventType::ConfigureNotify => {
               let (window_id, resize_width, resize_height) = event.resize_properties();

               if window_id != window.window_id {
                  continue;
               }

               if (resize_width != width) || (resize_height != height) {
                  new_width = resize_width;
                  new_height = resize_height;
               }
            },
            xcb::EventType::KeyPress => {
               exit = true;
               break;
            },
            _ => {}
         }
      }

      if exit {
         break;
      }
   }

   let end_time = time::precise_time_ns();

   println!("Time ns ................... : {:?}", end_time - start_time);
   println!("Cycles .................... : {:?}", counter);
   println!("FPS ....................... : {:?}", counter * 1000000000 / (end_time - start_time) );
}

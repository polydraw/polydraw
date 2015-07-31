#![cfg(target_os = "linux")]

extern crate polydraw;
extern crate rand;

use std::iter::repeat;
use rand::Rng;

use polydraw::os::xcb;
use polydraw::os::x11;
use polydraw::os::egl;
use polydraw::os::gl;
use polydraw::os::cl;

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

fn main() {
   let width: usize = 800;
   let height: usize = 450;

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

   let connection = match display.xcb_connection() {
      Ok(connection) => connection,
      Err(e) => {
         panic!(e.description);
      }
   };

   display.xcb_own_event_queue();

   let default_screen = display.default_screen();

   let scr = connection.screen_of_display(default_screen);

   print_screen_info(&scr);

   let window = connection.generate_id();

   println!("window .................... : {:?}", window);

   connection.create_window(
      window, &scr,
      0, 0, width as u16, height as u16,
   );

   connection.map_window(window);

   if !egl::bind_api(egl::API::OpenGL) {
      panic!("eglBindAPI failed");
   }

   let egl_d = egl::get_display(&display);
   let egl_display = egl_d.ptr;

   println!("egl display ............... : {:?}", egl_display);

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

   let surface = match egl::create_window_surface(&egl_d, &config, &window) {
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

   let mut data = repeat(0u8)
      .take(width * height * 3)
      .collect::<Vec<_>>();

   rand::thread_rng().fill_bytes(&mut data);

   let texture = gl::create_texture(width, height, &data);

   println!("GL texture ................ : {:?}", texture);

   let framebuffer = gl::create_framebuffer(texture);

   println!("GL framebuffer ............ : {:?}", framebuffer);

   loop {
      let event = match connection.wait_for_event() {
         None => {
            return;
         },
         Some(event) => event
      };

      let event_type = event.event_type();

      match event_type {
         xcb::EventType::KeyPress => {
            break;
         },
         xcb::EventType::Expose => {
            //gl::clear_color(0.0, 0.7, 1.0, 1.0);
            //gl::clear();

            gl::blit_framebuffer(framebuffer, width, height);

            gl::flush();

            match egl::swap_buffers(&egl_d, &surface) {
               Ok(_) => {},
               Err(e) => {
                  panic!(e.description);
               }
            };
         }
         _ => {}
      }
   }

   connection.destroy_window(window);
}

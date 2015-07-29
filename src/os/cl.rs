pub mod ffi {
   #![allow(non_camel_case_types)]

   use libc::{
      c_int, c_uint, c_void
   };

   pub type cl_int = c_int;
   pub type cl_uint = c_uint;

   pub type cl_platform_id = *mut c_void;

   pub const CL_SUCCESS:                                   cl_int = 0;
   pub const CL_DEVICE_NOT_FOUND:                          cl_int = -1;
   pub const CL_DEVICE_NOT_AVAILABLE:                      cl_int = -2;
   pub const CL_COMPILER_NOT_AVAILABLE:                    cl_int = -3;
   pub const CL_MEM_OBJECT_ALLOCATION_FAILURE:             cl_int = -4;
   pub const CL_OUT_OF_RESOURCES:                          cl_int = -5;
   pub const CL_OUT_OF_HOST_MEMORY:                        cl_int = -6;
   pub const CL_PROFILING_INFO_NOT_AVAILABLE:              cl_int = -7;
   pub const CL_MEM_COPY_OVERLAP:                          cl_int = -8;
   pub const CL_IMAGE_FORMAT_MISMATCH:                     cl_int = -9;
   pub const CL_IMAGE_FORMAT_NOT_SUPPORTED:                cl_int = -10;
   pub const CL_BUILD_PROGRAM_FAILURE:                     cl_int = -11;
   pub const CL_MAP_FAILURE:                               cl_int = -12;
   pub const CL_MISALIGNED_SUB_BUFFER_OFFSET:              cl_int = -13;
   pub const CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST: cl_int = -14;
   pub const CL_COMPILE_PROGRAM_FAILURE:                   cl_int = -15;
   pub const CL_LINKER_NOT_AVAILABLE:                      cl_int = -16;
   pub const CL_LINK_PROGRAM_FAILURE:                      cl_int = -17;
   pub const CL_DEVICE_PARTITION_FAILED:                   cl_int = -18;
   pub const CL_KERNEL_ARG_INFO_NOT_AVAILABLE:             cl_int = -19;

   pub const CL_INVALID_VALUE:                             cl_int = -30;
   pub const CL_INVALID_DEVICE_TYPE:                       cl_int = -31;
   pub const CL_INVALID_PLATFORM:                          cl_int = -32;
   pub const CL_INVALID_DEVICE:                            cl_int = -33;
   pub const CL_INVALID_CONTEXT:                           cl_int = -34;
   pub const CL_INVALID_QUEUE_PROPERTIES:                  cl_int = -35;
   pub const CL_INVALID_COMMAND_QUEUE:                     cl_int = -36;
   pub const CL_INVALID_HOST_PTR:                          cl_int = -37;
   pub const CL_INVALID_MEM_OBJECT:                        cl_int = -38;
   pub const CL_INVALID_IMAGE_FORMAT_DESCRIPTOR:           cl_int = -39;
   pub const CL_INVALID_IMAGE_SIZE:                        cl_int = -40;
   pub const CL_INVALID_SAMPLER:                           cl_int = -41;
   pub const CL_INVALID_BINARY:                            cl_int = -42;
   pub const CL_INVALID_BUILD_OPTIONS:                     cl_int = -43;
   pub const CL_INVALID_PROGRAM:                           cl_int = -44;
   pub const CL_INVALID_PROGRAM_EXECUTABLE:                cl_int = -45;
   pub const CL_INVALID_KERNEL_NAME:                       cl_int = -46;
   pub const CL_INVALID_KERNEL_DEFINITION:                 cl_int = -47;
   pub const CL_INVALID_KERNEL:                            cl_int = -48;
   pub const CL_INVALID_ARG_INDEX:                         cl_int = -49;
   pub const CL_INVALID_ARG_VALUE:                         cl_int = -50;
   pub const CL_INVALID_ARG_SIZE:                          cl_int = -51;
   pub const CL_INVALID_KERNEL_ARGS:                       cl_int = -52;
   pub const CL_INVALID_WORK_DIMENSION:                    cl_int = -53;
   pub const CL_INVALID_WORK_GROUP_SIZE:                   cl_int = -54;
   pub const CL_INVALID_WORK_ITEM_SIZE:                    cl_int = -55;
   pub const CL_INVALID_GLOBAL_OFFSET:                     cl_int = -56;
   pub const CL_INVALID_EVENT_WAIT_LIST:                   cl_int = -57;
   pub const CL_INVALID_EVENT:                             cl_int = -58;
   pub const CL_INVALID_OPERATION:                         cl_int = -59;
   pub const CL_INVALID_GL_OBJECT:                         cl_int = -60;
   pub const CL_INVALID_BUFFER_SIZE:                       cl_int = -61;
   pub const CL_INVALID_MIP_LEVEL:                         cl_int = -62;
   pub const CL_INVALID_GLOBAL_WORK_SIZE:                  cl_int = -63;
   pub const CL_INVALID_PROPERTY:                          cl_int = -64;
   pub const CL_INVALID_IMAGE_DESCRIPTOR:                  cl_int = -65;
   pub const CL_INVALID_COMPILER_OPTIONS:                  cl_int = -66;
   pub const CL_INVALID_LINKER_OPTIONS:                    cl_int = -67;
   pub const CL_INVALID_DEVICE_PARTITION_COUNT:            cl_int = -68;


   #[link(name="OpenCL")]
   extern "C" {
      pub fn clGetPlatformIDs(
         num_entries: cl_uint,
         platforms: *mut cl_platform_id,
         num_platforms: *mut cl_uint
      ) -> cl_int;
   }
}

use std::ptr;
use std::iter::repeat;

use ::error::{RuntimeError, ErrorKind};

pub struct Platform {
   pub ptr: ffi::cl_platform_id
}

pub fn get_platforms() -> Result<Vec<Platform>, RuntimeError> {
   let mut num_platforms = 0 as ffi::cl_uint;

   let result = unsafe {
      ffi::clGetPlatformIDs(
         0,
         ptr::null_mut(),
         &mut num_platforms
      )
   };

   if result != ffi::CL_SUCCESS {
      return Err(RuntimeError::new(
         ErrorKind::CL,
         "Getting number of OpenCL platforms failed".to_string()
      ));
   }

   let mut platform_ids = repeat(0 as ffi::cl_platform_id)
      .take(num_platforms as usize)
      .collect::<Vec<_>>();

   let result = unsafe {
      ffi::clGetPlatformIDs(
         num_platforms,
         platform_ids.as_mut_ptr(),
         ptr::null_mut()
      )
   };

   if result != ffi::CL_SUCCESS {
      return Err(RuntimeError::new(
         ErrorKind::CL,
         "Getting OpenCL platforms failed".to_string()
      ));
   }

   Ok(
      platform_ids.iter()
         .map(|platform_id| { Platform { ptr: *platform_id } })
         .collect()
   )
}


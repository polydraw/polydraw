pub mod ffi {
   #![allow(non_camel_case_types)]

   use libc::{
      c_int, c_uint, c_ulong, c_void, size_t
   };

   pub type cl_int = c_int;
   pub type cl_uint = c_uint;
   pub type cl_ulong = c_ulong;

   pub type cl_platform_info = cl_uint;
   pub type cl_device_info = cl_uint;
   pub type cl_bitfield = cl_ulong;
   pub type cl_device_type = cl_bitfield;

   pub type cl_platform_id = *mut c_void;
   pub type cl_device_id = *mut c_void;

   pub const CL_SUCCESS:                                      cl_int = 0;
   pub const CL_DEVICE_NOT_FOUND:                             cl_int = -1;
   pub const CL_DEVICE_NOT_AVAILABLE:                         cl_int = -2;
   pub const CL_COMPILER_NOT_AVAILABLE:                       cl_int = -3;
   pub const CL_MEM_OBJECT_ALLOCATION_FAILURE:                cl_int = -4;
   pub const CL_OUT_OF_RESOURCES:                             cl_int = -5;
   pub const CL_OUT_OF_HOST_MEMORY:                           cl_int = -6;
   pub const CL_PROFILING_INFO_NOT_AVAILABLE:                 cl_int = -7;
   pub const CL_MEM_COPY_OVERLAP:                             cl_int = -8;
   pub const CL_IMAGE_FORMAT_MISMATCH:                        cl_int = -9;
   pub const CL_IMAGE_FORMAT_NOT_SUPPORTED:                   cl_int = -10;
   pub const CL_BUILD_PROGRAM_FAILURE:                        cl_int = -11;
   pub const CL_MAP_FAILURE:                                  cl_int = -12;
   pub const CL_MISALIGNED_SUB_BUFFER_OFFSET:                 cl_int = -13;
   pub const CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST:    cl_int = -14;
   pub const CL_COMPILE_PROGRAM_FAILURE:                      cl_int = -15;
   pub const CL_LINKER_NOT_AVAILABLE:                         cl_int = -16;
   pub const CL_LINK_PROGRAM_FAILURE:                         cl_int = -17;
   pub const CL_DEVICE_PARTITION_FAILED:                      cl_int = -18;
   pub const CL_KERNEL_ARG_INFO_NOT_AVAILABLE:                cl_int = -19;

   pub const CL_INVALID_VALUE:                                cl_int = -30;
   pub const CL_INVALID_DEVICE_TYPE:                          cl_int = -31;
   pub const CL_INVALID_PLATFORM:                             cl_int = -32;
   pub const CL_INVALID_DEVICE:                               cl_int = -33;
   pub const CL_INVALID_CONTEXT:                              cl_int = -34;
   pub const CL_INVALID_QUEUE_PROPERTIES:                     cl_int = -35;
   pub const CL_INVALID_COMMAND_QUEUE:                        cl_int = -36;
   pub const CL_INVALID_HOST_PTR:                             cl_int = -37;
   pub const CL_INVALID_MEM_OBJECT:                           cl_int = -38;
   pub const CL_INVALID_IMAGE_FORMAT_DESCRIPTOR:              cl_int = -39;
   pub const CL_INVALID_IMAGE_SIZE:                           cl_int = -40;
   pub const CL_INVALID_SAMPLER:                              cl_int = -41;
   pub const CL_INVALID_BINARY:                               cl_int = -42;
   pub const CL_INVALID_BUILD_OPTIONS:                        cl_int = -43;
   pub const CL_INVALID_PROGRAM:                              cl_int = -44;
   pub const CL_INVALID_PROGRAM_EXECUTABLE:                   cl_int = -45;
   pub const CL_INVALID_KERNEL_NAME:                          cl_int = -46;
   pub const CL_INVALID_KERNEL_DEFINITION:                    cl_int = -47;
   pub const CL_INVALID_KERNEL:                               cl_int = -48;
   pub const CL_INVALID_ARG_INDEX:                            cl_int = -49;
   pub const CL_INVALID_ARG_VALUE:                            cl_int = -50;
   pub const CL_INVALID_ARG_SIZE:                             cl_int = -51;
   pub const CL_INVALID_KERNEL_ARGS:                          cl_int = -52;
   pub const CL_INVALID_WORK_DIMENSION:                       cl_int = -53;
   pub const CL_INVALID_WORK_GROUP_SIZE:                      cl_int = -54;
   pub const CL_INVALID_WORK_ITEM_SIZE:                       cl_int = -55;
   pub const CL_INVALID_GLOBAL_OFFSET:                        cl_int = -56;
   pub const CL_INVALID_EVENT_WAIT_LIST:                      cl_int = -57;
   pub const CL_INVALID_EVENT:                                cl_int = -58;
   pub const CL_INVALID_OPERATION:                            cl_int = -59;
   pub const CL_INVALID_GL_OBJECT:                            cl_int = -60;
   pub const CL_INVALID_BUFFER_SIZE:                          cl_int = -61;
   pub const CL_INVALID_MIP_LEVEL:                            cl_int = -62;
   pub const CL_INVALID_GLOBAL_WORK_SIZE:                     cl_int = -63;
   pub const CL_INVALID_PROPERTY:                             cl_int = -64;
   pub const CL_INVALID_IMAGE_DESCRIPTOR:                     cl_int = -65;
   pub const CL_INVALID_COMPILER_OPTIONS:                     cl_int = -66;
   pub const CL_INVALID_LINKER_OPTIONS:                       cl_int = -67;
   pub const CL_INVALID_DEVICE_PARTITION_COUNT:               cl_int = -68;

   pub const CL_PLATFORM_PROFILE:                   cl_platform_info = 0x0900;
   pub const CL_PLATFORM_VERSION:                   cl_platform_info = 0x0901;
   pub const CL_PLATFORM_NAME:                      cl_platform_info = 0x0902;
   pub const CL_PLATFORM_VENDOR:                    cl_platform_info = 0x0903;
   pub const CL_PLATFORM_EXTENSIONS:                cl_platform_info = 0x0904;

   pub const CL_DEVICE_TYPE_DEFAULT:                  cl_device_type = 1 << 0;
   pub const CL_DEVICE_TYPE_CPU:                      cl_device_type = 1 << 1;
   pub const CL_DEVICE_TYPE_GPU:                      cl_device_type = 1 << 2;
   pub const CL_DEVICE_TYPE_ACCELERATOR:              cl_device_type = 1 << 3;
   pub const CL_DEVICE_TYPE_CUSTOM:                   cl_device_type = 1 << 4;
   pub const CL_DEVICE_TYPE_ALL:                      cl_device_type = 0xFFFFFFFF;

   pub const CL_DEVICE_TYPE:                          cl_device_info = 0x1000;
   pub const CL_DEVICE_VENDOR_ID:                     cl_device_info = 0x1001;
   pub const CL_DEVICE_MAX_COMPUTE_UNITS:             cl_device_info = 0x1002;
   pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS:      cl_device_info = 0x1003;
   pub const CL_DEVICE_MAX_WORK_GROUP_SIZE:           cl_device_info = 0x1004;
   pub const CL_DEVICE_MAX_WORK_ITEM_SIZES:           cl_device_info = 0x1005;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR:   cl_device_info = 0x1006;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT:  cl_device_info = 0x1007;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT:    cl_device_info = 0x1008;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG:   cl_device_info = 0x1009;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT:  cl_device_info = 0x100A;
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: cl_device_info = 0x100B;
   pub const CL_DEVICE_MAX_CLOCK_FREQUENCY:           cl_device_info = 0x100C;
   pub const CL_DEVICE_ADDRESS_BITS:                  cl_device_info = 0x100D;
   pub const CL_DEVICE_MAX_READ_IMAGE_ARGS:           cl_device_info = 0x100E;
   pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS:          cl_device_info = 0x100F;
   pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE:            cl_device_info = 0x1010;
   pub const CL_DEVICE_IMAGE2D_MAX_WIDTH:             cl_device_info = 0x1011;
   pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT:            cl_device_info = 0x1012;
   pub const CL_DEVICE_IMAGE3D_MAX_WIDTH:             cl_device_info = 0x1013;
   pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT:            cl_device_info = 0x1014;
   pub const CL_DEVICE_IMAGE3D_MAX_DEPTH:             cl_device_info = 0x1015;
   pub const CL_DEVICE_IMAGE_SUPPORT:                 cl_device_info = 0x1016;
   pub const CL_DEVICE_MAX_PARAMETER_SIZE:            cl_device_info = 0x1017;
   pub const CL_DEVICE_MAX_SAMPLERS:                  cl_device_info = 0x1018;
   pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN:           cl_device_info = 0x1019;
   pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE:      cl_device_info = 0x101A;
   pub const CL_DEVICE_SINGLE_FP_CONFIG:              cl_device_info = 0x101B;
   pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE:         cl_device_info = 0x101C;
   pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE:     cl_device_info = 0x101D;
   pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE:         cl_device_info = 0x101E;
   pub const CL_DEVICE_GLOBAL_MEM_SIZE:               cl_device_info = 0x101F;
   pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE:      cl_device_info = 0x1020;
   pub const CL_DEVICE_MAX_CONSTANT_ARGS:             cl_device_info = 0x1021;
   pub const CL_DEVICE_LOCAL_MEM_TYPE:                cl_device_info = 0x1022;
   pub const CL_DEVICE_LOCAL_MEM_SIZE:                cl_device_info = 0x1023;
   pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT:      cl_device_info = 0x1024;
   pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION:    cl_device_info = 0x1025;
   pub const CL_DEVICE_ENDIAN_LITTLE:                 cl_device_info = 0x1026;
   pub const CL_DEVICE_AVAILABLE:                     cl_device_info = 0x1027;
   pub const CL_DEVICE_COMPILER_AVAILABLE:            cl_device_info = 0x1028;
   pub const CL_DEVICE_EXECUTION_CAPABILITIES:        cl_device_info = 0x1029;
   pub const CL_DEVICE_QUEUE_PROPERTIES:              cl_device_info = 0x102A;
   pub const CL_DEVICE_NAME:                          cl_device_info = 0x102B;
   pub const CL_DEVICE_VENDOR:                        cl_device_info = 0x102C;
   pub const CL_DRIVER_VERSION:                       cl_device_info = 0x102D;
   pub const CL_DEVICE_PROFILE:                       cl_device_info = 0x102E;
   pub const CL_DEVICE_VERSION:                       cl_device_info = 0x102F;
   pub const CL_DEVICE_EXTENSIONS:                    cl_device_info = 0x1030;
   pub const CL_DEVICE_PLATFORM:                      cl_device_info = 0x1031;
   pub const CL_DEVICE_DOUBLE_FP_CONFIG:              cl_device_info = 0x1032;
/* 0x1033 reserved for CL_DEVICE_HALF_FP_CONFIG */
   pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF:   cl_device_info = 0x1034;
   pub const CL_DEVICE_HOST_UNIFIED_MEMORY:           cl_device_info = 0x1035;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR:      cl_device_info = 0x1036;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT:     cl_device_info = 0x1037;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT:       cl_device_info = 0x1038;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG:      cl_device_info = 0x1039;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT:     cl_device_info = 0x103A;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE:    cl_device_info = 0x103B;
   pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF:      cl_device_info = 0x103C;
   pub const CL_DEVICE_OPENCL_C_VERSION:              cl_device_info = 0x103D;
   pub const CL_DEVICE_LINKER_AVAILABLE:              cl_device_info = 0x103E;
   pub const CL_DEVICE_BUILT_IN_KERNELS:              cl_device_info = 0x103F;
   pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE:         cl_device_info = 0x1040;
   pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE:          cl_device_info = 0x1041;
   pub const CL_DEVICE_PARENT_DEVICE:                 cl_device_info = 0x1042;
   pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES:     cl_device_info = 0x1043;
   pub const CL_DEVICE_PARTITION_PROPERTIES:          cl_device_info = 0x1044;
   pub const CL_DEVICE_PARTITION_AFFINITY_DOMAIN:     cl_device_info = 0x1045;
   pub const CL_DEVICE_PARTITION_TYPE:                cl_device_info = 0x1046;
   pub const CL_DEVICE_REFERENCE_COUNT:               cl_device_info = 0x1047;
   pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC:   cl_device_info = 0x1048;
   pub const CL_DEVICE_PRINTF_BUFFER_SIZE:            cl_device_info = 0x1049;
   pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT:         cl_device_info = 0x104A;
   pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT:  cl_device_info = 0x104B;

   #[link(name="OpenCL")]
   extern "C" {
      pub fn clGetPlatformIDs(
         num_entries: cl_uint,
         platforms: *mut cl_platform_id,
         num_platforms: *mut cl_uint
      ) -> cl_int;

      pub fn clGetPlatformInfo(
         platform: cl_platform_id,
         param_name: cl_platform_info,
         param_value_size: size_t,
         param_value: *mut c_void,
         param_value_size_ret: *mut size_t
      ) -> cl_int;

      pub fn clGetDeviceIDs(
         platform: cl_platform_id,
         device_type: cl_device_type,
         num_entries: cl_uint,
         devices: *mut cl_device_id,
         num_devices: *mut cl_uint
      ) -> cl_int;

      pub fn clGetDeviceInfo(
         device: cl_device_id,
         param_name: cl_device_info,
         param_value_size: size_t,
         param_value: *mut c_void,
         param_value_size_ret: *mut size_t
      ) -> cl_int;
   }
}

use std::ptr;
use std::iter::repeat;
use super::utils::from_cstr;

use libc::{
   c_char, c_void
};

use ::error::{RuntimeError, ErrorKind};

pub fn platforms() -> Result<Vec<Platform>, RuntimeError> {
   let mut num_platforms = 0;

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

pub struct Platform {
   pub ptr: ffi::cl_platform_id
}

impl Platform {
   pub fn profile(&self) -> Result<String, RuntimeError> {
      self.info(ffi::CL_PLATFORM_PROFILE)
   }

   pub fn version(&self) -> Result<String, RuntimeError> {
      self.info(ffi::CL_PLATFORM_VERSION)
   }

   pub fn name(&self) -> Result<String, RuntimeError> {
      self.info(ffi::CL_PLATFORM_NAME)
   }

   pub fn vendor(&self) -> Result<String, RuntimeError> {
      self.info(ffi::CL_PLATFORM_VENDOR)
   }

   pub fn extensions(&self) -> Result<String, RuntimeError> {
      self.info(ffi::CL_PLATFORM_EXTENSIONS)
   }

   fn info(&self, platform_info: ffi::cl_platform_info) -> Result<String, RuntimeError> {
      let mut size = 0;

      let result = unsafe {
         ffi::clGetPlatformInfo(
            self.ptr,
            platform_info,
            0,
            ptr::null_mut(),
            &mut size
         )
      };

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            "Getting size of OpenCL platform info string failed".to_string()
         ));
      }

      let mut cbuf = repeat(0u8)
         .take(size as usize)
         .collect::<Vec<_>>();

      let result = unsafe {
         ffi::clGetPlatformInfo(
            self.ptr,
            platform_info,
            size,
            cbuf.as_mut_ptr() as *mut c_void,
            ptr::null_mut()
         )
      };

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            "Getting OpenCL platform info string failed".to_string()
         ));
      }

      Ok(
         from_cstr(cbuf.as_ptr() as *const c_char)
      )
   }

   pub fn default_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_DEFAULT)
   }

   pub fn cpu_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_CPU)
   }

   pub fn gpu_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_GPU)
   }

   pub fn accelerator_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_ACCELERATOR)
   }

   pub fn custom_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_CUSTOM)
   }

   pub fn all_devices(&self) -> Result<Vec<Device>, RuntimeError> {
      self.devices(ffi::CL_DEVICE_TYPE_ALL)
   }

   fn devices(&self, device_type: ffi::cl_device_type) -> Result<Vec<Device>, RuntimeError> {
      let mut num_devices = 0;

      let result = unsafe {
         ffi::clGetDeviceIDs(
            self.ptr,
            device_type,
            0,
            ptr::null_mut(),
            &mut num_devices
         )
      };

      if result == ffi::CL_DEVICE_NOT_FOUND {
         return Ok(vec!());
      }

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            format!("Getting number of OpenCL devices failed {}", result).to_string()
         ));
      }

      let mut device_ids = repeat(0 as ffi::cl_device_id)
         .take(num_devices as usize)
         .collect::<Vec<_>>();


      let result = unsafe {
         ffi::clGetDeviceIDs(
            self.ptr,
            device_type,
            num_devices,
            device_ids.as_mut_ptr(),
            ptr::null_mut()
         )
      };

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            "Getting OpenCL devices failed".to_string()
         ));
      }

      Ok(
         device_ids.iter()
            .map(|device_id| { Device { ptr: *device_id } })
            .collect()
      )
   }
}

pub struct Device {
   pub ptr: ffi::cl_device_id
}

impl Device {
   pub fn driver_version(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DRIVER_VERSION)
   }

   pub fn built_in_kernels(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_BUILT_IN_KERNELS)
   }

   pub fn extensions(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_EXTENSIONS)
   }

   pub fn name(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_NAME)
   }

   pub fn opencl_c_version(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_OPENCL_C_VERSION)
   }

   pub fn profile(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_PROFILE)
   }

   pub fn vendor(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_VENDOR)
   }

   pub fn version(&self) -> Result<String, RuntimeError> {
      self.info_string(ffi::CL_DEVICE_VERSION)
   }

   pub fn available(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_AVAILABLE)
   }

   pub fn compiler_available(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_COMPILER_AVAILABLE)
   }

   pub fn linker_available(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_LINKER_AVAILABLE)
   }

   pub fn endian_little(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_ENDIAN_LITTLE)
   }

   pub fn error_correction(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_ERROR_CORRECTION_SUPPORT)
   }

   pub fn unified_memory(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_HOST_UNIFIED_MEMORY)
   }

   pub fn image_support(&self) -> Result<bool, RuntimeError> {
      self.info_bool(ffi::CL_DEVICE_IMAGE_SUPPORT)
   }

   #[inline]
   fn info_bool(&self, device_info: ffi::cl_device_info) -> Result<bool, RuntimeError> {
      match self.info(device_info) {
         Ok(cbuf) => Ok(cbuf[0] == 1),
         Err(e) => Err(e)
      }
   }

   #[inline]
   fn info_string(&self, device_info: ffi::cl_device_info) -> Result<String, RuntimeError> {
      match self.info(device_info) {
         Ok(cbuf) => Ok(from_cstr(cbuf.as_ptr() as *const c_char)),
         Err(e) => Err(e)
      }
   }

   fn info(&self, device_info: ffi::cl_device_info) -> Result<Vec<u8>, RuntimeError> {
      let mut size = 0;

      let result = unsafe {
         ffi::clGetDeviceInfo(
            self.ptr,
            device_info,
            0,
            ptr::null_mut(),
            &mut size
         )
      };

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            "Getting size of OpenCL device param value failed".to_string()
         ));
      }

      let mut cbuf = repeat(0u8)
         .take(size as usize)
         .collect::<Vec<_>>();

      let result = unsafe {
         ffi::clGetDeviceInfo(
            self.ptr,
            device_info,
            size,
            cbuf.as_mut_ptr() as *mut c_void,
            ptr::null_mut()
         )
      };

      if result != ffi::CL_SUCCESS {
         return Err(RuntimeError::new(
            ErrorKind::CL,
            "Getting OpenCL device param value failed".to_string()
         ));
      }

      Ok(cbuf)
   }
}

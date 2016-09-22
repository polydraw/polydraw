#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

use libc::{
   c_char, c_int, c_uint, c_ulong, c_void, size_t
};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

pub type intptr_t = isize;

pub type cl_int = c_int;
pub type cl_uint = c_uint;
pub type cl_ulong = c_ulong;

pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_context_properties = intptr_t;
pub type cl_queue_properties = cl_bitfield;
pub type cl_mem_flags = cl_bitfield;

pub type cl_platform_id = *mut c_void;
pub type cl_device_id = *mut c_void;

pub enum _cl_context { }
pub type cl_context = *mut _cl_context;

pub enum _cl_command_queue { }
pub type cl_command_queue = *mut _cl_command_queue;

pub enum _cl_mem { }
pub type cl_mem = *mut _cl_mem;

pub enum _cl_program { }
pub type cl_program = *mut _cl_program;

pub enum _cl_kernel { }
pub type cl_kernel = *mut _cl_kernel;

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

pub const CL_MEM_READ_WRITE:                          cl_bitfield = 1 << 0;
pub const CL_MEM_WRITE_ONLY:                          cl_bitfield = 1 << 1;
pub const CL_MEM_READ_ONLY:                           cl_bitfield = 1 << 2;
pub const CL_MEM_USE_HOST_PTR:                        cl_bitfield = 1 << 3;
pub const CL_MEM_ALLOC_HOST_PTR:                      cl_bitfield = 1 << 4;
pub const CL_MEM_COPY_HOST_PTR:                       cl_bitfield = 1 << 5;
// RESERVED                                           cl_bitfield = 1 << 6;
pub const CL_MEM_HOST_WRITE_ONLY:                     cl_bitfield = 1 << 7;
pub const CL_MEM_HOST_READ_ONLY:                      cl_bitfield = 1 << 8;
pub const CL_MEM_HOST_NO_ACCESS:                      cl_bitfield = 1 << 9;
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER:               cl_bitfield = 1 << 10;
pub const CL_MEM_SVM_ATOMICS:                         cl_bitfield = 1 << 11;
pub const CL_MEM_KERNEL_READ_AND_WRITE:               cl_bitfield = 1 << 12;


pub type CL_CREATE_CONTEXT_CALLBACK = unsafe extern "C" fn(
   errinfo: *const c_char,
   private_info: *const c_void,
   cb: size_t,
   user_data: *mut c_void
);

pub type CL_BUILD_PROGRAM_CALLBACK = unsafe extern "C" fn(
   program: cl_program,
   user_data: *mut c_void
);


static mut clGetPlatformIDsPtr:                             FnPtr = NULL_PTR;
static mut clGetPlatformInfoPtr:                            FnPtr = NULL_PTR;
static mut clGetDeviceIDsPtr:                               FnPtr = NULL_PTR;
static mut clGetDeviceInfoPtr:                              FnPtr = NULL_PTR;
static mut clCreateContextPtr:                              FnPtr = NULL_PTR;
static mut clCreateCommandQueueWithPropertiesPtr:           FnPtr = NULL_PTR;
static mut clCreateBufferPtr:                               FnPtr = NULL_PTR;
static mut clCreateProgramWithSourcePtr:                    FnPtr = NULL_PTR;
static mut clBuildProgramPtr:                               FnPtr = NULL_PTR;
static mut clCreateKernelPtr:                               FnPtr = NULL_PTR;


#[inline]
pub unsafe fn clGetPlatformIDs(
   num_entries: cl_uint,
   platforms: *mut cl_platform_id,
   num_platforms: *mut cl_uint
) -> cl_int {
   mem::transmute::<_, extern "system" fn(
      cl_uint, *mut cl_platform_id, *mut cl_uint
   ) -> cl_int>(clGetPlatformIDsPtr)(
      num_entries, platforms, num_platforms
   )
}

#[inline]
pub unsafe fn clGetPlatformInfo(
   platform: cl_platform_id,
   param_name: cl_platform_info,
   param_value_size: size_t,
   param_value: *mut c_void,
   param_value_size_ret: *mut size_t
) -> cl_int {
   mem::transmute::<_, extern "system" fn(
      cl_platform_id, cl_platform_info, size_t, *mut c_void, *mut size_t
   ) -> cl_int>(clGetPlatformInfoPtr)(
      platform, param_name, param_value_size, param_value, param_value_size_ret
   )
}

#[inline]
pub unsafe fn clGetDeviceIDs(
   platform: cl_platform_id,
   device_type: cl_device_type,
   num_entries: cl_uint,
   devices: *mut cl_device_id,
   num_devices: *mut cl_uint
) -> cl_int {
   mem::transmute::<_, extern "system" fn(
      cl_platform_id, cl_device_type, cl_uint, *mut cl_device_id, *mut cl_uint
   ) -> cl_int>(clGetDeviceIDsPtr)(
      platform, device_type, num_entries, devices, num_devices
   )
}

#[inline]
pub unsafe fn clGetDeviceInfo(
   device: cl_device_id,
   param_name: cl_device_info,
   param_value_size: size_t,
   param_value: *mut c_void,
   param_value_size_ret: *mut size_t
) -> cl_int {
   mem::transmute::<_, extern "system" fn(
      cl_device_id, cl_device_info, size_t, *mut c_void, *mut size_t
   ) -> cl_int>(clGetDeviceInfoPtr)(
      device, param_name, param_value_size, param_value, param_value_size_ret
   )
}

#[inline]
pub unsafe fn clCreateContext(
   properties: *const cl_context_properties,
   num_devices: cl_uint,
   devices: *const cl_device_id,
   pfn_notify: Option<CL_CREATE_CONTEXT_CALLBACK>,
   user_data: *mut c_void,
   errcode_ret: *mut cl_int
) -> cl_context {
   mem::transmute::<_, extern "system" fn(
      *const cl_context_properties, cl_uint, *const cl_device_id, Option<CL_CREATE_CONTEXT_CALLBACK>, *mut c_void, *mut cl_int
   ) -> cl_context>(clCreateContextPtr)(
      properties, num_devices, devices, pfn_notify, user_data, errcode_ret
   )
}

#[inline]
pub unsafe fn clCreateCommandQueueWithProperties(
   context: cl_context,
   device: cl_device_id,
   properties: *const cl_queue_properties,
   errcode_ret: *mut cl_int
) -> cl_command_queue {
   mem::transmute::<_, extern "system" fn(
      cl_context, cl_device_id, *const cl_queue_properties, *mut cl_int
   ) -> cl_command_queue>(clCreateCommandQueueWithPropertiesPtr)(
      context, device, properties, errcode_ret
   )
}

#[inline]
pub unsafe fn clCreateBuffer(
   context: cl_context,
   flags: cl_mem_flags,
   size: size_t,
   host_ptr: *mut c_void,
   errcode_ret: *mut cl_int
) -> cl_mem {
   mem::transmute::<_, extern "system" fn(
      cl_context, cl_mem_flags, size_t, *mut c_void, *mut cl_int
   ) -> cl_mem>(clCreateBufferPtr)(
      context, flags, size, host_ptr, errcode_ret
   )
}

#[inline]
pub unsafe fn clCreateProgramWithSource(
   context: cl_context,
   count: cl_uint,
   strings: *mut *const c_char,
   lengths: *const size_t,
   errcode_ret: *mut cl_int
) -> cl_program {
   mem::transmute::<_, extern "system" fn(
      cl_context, cl_uint, *mut *const c_char, *const size_t, *mut cl_int
   ) -> cl_program>(clCreateProgramWithSourcePtr)(
      context, count, strings, lengths, errcode_ret
   )
}


#[inline]
pub unsafe fn clBuildProgram(
   program: cl_program,
   num_devices: cl_uint,
   device_list: *const cl_device_id,
   options: *const c_char,
   pfn_notify: Option<CL_BUILD_PROGRAM_CALLBACK>,
   user_data: *mut c_void
) -> cl_int {
   mem::transmute::<_, extern "system" fn(
      cl_program, cl_uint, *const cl_device_id, *const c_char, Option<CL_BUILD_PROGRAM_CALLBACK>, *mut c_void
   ) -> cl_int>(clBuildProgramPtr)(
      program, num_devices, device_list, options, pfn_notify, user_data
   )
}

#[inline]
pub unsafe fn clCreateKernel(
   program: cl_program,
   kernel_name: *const c_char,
   errcode_ret: *mut cl_int
) -> cl_kernel {
   mem::transmute::<_, extern "system" fn(
      cl_program, *const c_char, *mut cl_int
   ) -> cl_kernel>(clCreateKernelPtr)(
      program, kernel_name, errcode_ret
   )
}


pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   clGetPlatformIDsPtr = loader.load("clGetPlatformIDs");
   clGetPlatformInfoPtr = loader.load("clGetPlatformInfo");
   clGetDeviceIDsPtr = loader.load("clGetDeviceIDs");
   clGetDeviceInfoPtr = loader.load("clGetDeviceInfo");
   clCreateContextPtr = loader.load("clCreateContext");
   clCreateCommandQueueWithPropertiesPtr = loader.load("clCreateCommandQueueWithProperties");
   clCreateBufferPtr = loader.load("clCreateBuffer");
   clCreateProgramWithSourcePtr = loader.load("clCreateProgramWithSource");
   clBuildProgramPtr = loader.load("clBuildProgram");
   clCreateKernelPtr = loader.load("clCreateKernel");

   are_functions_loaded()
}

unsafe fn are_functions_loaded() -> bool {
   clGetPlatformIDsPtr != NULL_PTR &&
   clGetPlatformInfoPtr != NULL_PTR &&
   clGetDeviceIDsPtr != NULL_PTR &&
   clGetDeviceInfoPtr != NULL_PTR &&
   clCreateContextPtr != NULL_PTR &&
   clCreateCommandQueueWithPropertiesPtr != NULL_PTR &&
   clCreateBufferPtr != NULL_PTR &&
   clCreateProgramWithSourcePtr != NULL_PTR &&
   clBuildProgramPtr != NULL_PTR &&
   clCreateKernelPtr != NULL_PTR
}

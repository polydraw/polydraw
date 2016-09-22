pub mod ffi;

use std::ptr;
use std::iter::repeat;

use libc::{c_char, c_void};

use error::{RuntimeError, ErrorKind};

use super::utils::string::from_cstr;
use super::utils::fn_ptr::FnPtrLoader;

#[inline]
pub fn load<T: FnPtrLoader>(loader: &T) {
   unsafe {
      ffi::load_functions(loader)
   };
}

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

use std::ffi::NulError;

#[derive(Clone, Debug)]
pub struct RuntimeError {
   pub kind: ErrorKind,
   pub description: String,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
   InvalidInput,
   Xlib,
   XCB,
   EGL,
   CL,
   Other,
}

impl RuntimeError {
   pub fn new(kind: ErrorKind, description: String) -> Self {
      RuntimeError {
         kind: kind,
         description: description,
      }
   }
}

impl From<NulError> for RuntimeError {
   fn from(_: NulError) -> RuntimeError {
      RuntimeError::new(
         ErrorKind::InvalidInput,
         "Data provided contains a nul byte".to_string()
      )
   }
}

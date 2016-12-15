pub mod ffi;

use std::os::raw::{c_void, c_int};

use std::fmt;
use std::ptr;
use std::ffi::CString;

use data::FloatPoint;

use super::DynLibrary;
use super::utils::fn_ptr::FnPtrLibrary;


pub struct FreeType {
   pub dyn_lib: DynLibrary,
   pub ft_lib: ffi::FT_Library,
}

impl FreeType {
   pub fn new() -> Self {
      let dyn_lib = FreeType::load_library();

      unsafe {
         ffi::load_functions(&dyn_lib);

         let mut ft_lib: ffi::FT_Library = ptr::null_mut();

         ffi::FT_Init_FreeType(&mut ft_lib);

         FreeType {
            ft_lib: ft_lib,
            dyn_lib: dyn_lib,
         }
      }
   }

   #[inline]
   #[cfg(target_os = "linux")]
   pub fn load_library() -> DynLibrary {
      DynLibrary::open("libfreetype.so.6").unwrap()
   }

   #[inline]
   #[cfg(target_os = "windows")]
   pub fn load_library() -> DynLibrary {
      DynLibrary::open("freetype.dll").unwrap()
   }

   pub fn load_face(&self, path: &str) -> Face {
      let cpath = CString::new(path).unwrap();

      let mut ft_face: ffi::FT_Face = ptr::null_mut();

      unsafe {
         ffi::FT_New_Face(self.ft_lib, cpath.as_ptr() as *const _, 0, &mut ft_face);
      }

      Face::new(ft_face, String::from(path))
   }
}

impl Drop for FreeType {
   fn drop (&mut self) {
      unsafe {
         ffi::FT_Done_FreeType(self.ft_lib);
      }
   }
}


pub struct Face {
   pub ft_face: ffi::FT_Face,
   pub path: String,
}

impl Face {
   pub fn new(ft_face: ffi::FT_Face, path: String) -> Self {
      unsafe {
         ffi::FT_Set_Pixel_Sizes(ft_face, 0, 2048);
      }

      Face {
         ft_face: ft_face,
         path: path,
      }
   }

   pub fn text(&self, string: &str, steps: usize) -> Vec<Vec<Vec<FloatPoint>>> {
      let funcs = ffi::FT_Outline_Funcs {
         move_to: Some(move_to),
         line_to: Some(line_to),
         conic_to: Some(conic_to),
         cubic_to: Some(cubic_to),
         shift: 0,
         delta: 0,
      };

      let mut result = Vec::new();

      let mut offset = 0.0;

      let mut previous_index = 0;

      for ch in string.chars() {
         let mut points: Box<CharPoints> = Box::new(CharPoints::new(steps));

         unsafe {
            ffi::FT_Load_Char(self.ft_face, ch as u32, ffi::FT_LOAD_DEFAULT);

            let slot = (*self.ft_face).glyph as ffi::FT_GlyphSlot;

            let current_index = ffi::FT_Get_Char_Index(self.ft_face, ch as u32);

            if previous_index != 0 {
               let mut delta = ffi::FT_Vector::default();

               ffi::FT_Get_Kerning(
                  self.ft_face,
                  previous_index,
                  current_index,
                  ffi::FT_KERNING_DEFAULT,
                  &mut delta
               );

               offset += delta.x as f64;
            }

            ffi::FT_Outline_Decompose(
               &mut (*slot).outline,
               &funcs,
               &mut *points as *mut _ as *mut c_void
            );

            let contours = points.order_points(offset);

            if contours.len() > 0 {
               result.push(contours);
            }

            offset += (*slot).metrics.horiAdvance as f64;

            previous_index = current_index;
         }
      }

      result
   }
}

impl Clone for Face {
   fn clone(&self) -> Self {
      unsafe {
         ffi::FT_Reference_Face(self.ft_face);
      }

      Face {
         ft_face: self.ft_face,
         path: self.path.clone(),
      }
   }
}

impl Drop for Face {
   fn drop (&mut self) {
      unsafe {
         ffi::FT_Done_Face(self.ft_face);
      }
   }
}

impl fmt::Debug for Face {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "font_face! {:?}", self.path)
   }
}


unsafe extern "C" fn move_to(
   to: *const ffi::FT_Vector,
   user: *mut c_void
) -> c_int {
   let points: &mut CharPoints = &mut *(user as *mut CharPoints);

   points.move_to(pt(to));

   0
}

unsafe extern "C" fn line_to(
   to: *const ffi::FT_Vector,
   user: *mut c_void
) -> c_int {
   let points: &mut CharPoints = &mut *(user as *mut CharPoints);

   points.line_to(pt(to));

   0
}

unsafe extern "C" fn conic_to(
   ctrl: *const ffi::FT_Vector,
   to: *const ffi::FT_Vector,
   user: *mut c_void
) -> c_int {
   let points: &mut CharPoints = &mut *(user as *mut CharPoints);

   points.conic_to(pt(ctrl), pt(to));

   0
}

unsafe extern "C" fn cubic_to(
   ctrl1: *const ffi::FT_Vector,
   ctrl2: *const ffi::FT_Vector,
   to: *const ffi::FT_Vector,
   user: *mut c_void
) -> c_int {
   let points: &mut CharPoints = &mut *(user as *mut CharPoints);

   points.cubic_to(pt(ctrl1), pt(ctrl2), pt(to));

   0
}

unsafe fn pt(ft_vec: *const ffi::FT_Vector) -> FloatPoint {
   FloatPoint::new((*ft_vec).x as f64, (*ft_vec).y as f64)
}


#[derive(Debug)]
struct CharPoints {
   points: Vec<Vec<FloatPoint>>,
   steps: usize,
}

impl CharPoints {
   pub fn new(steps: usize) -> Self {
      CharPoints {
         points: Vec::new(),
         steps: steps,
      }
   }

   #[inline]
   pub fn move_to(&mut self, to: FloatPoint) {
      self.points.push(Vec::new());

      self.push(to);
   }

   #[inline]
   pub fn line_to(&mut self, to: FloatPoint) {
      self.push(to);
   }

   #[inline]
   pub fn conic_to(&mut self, ctrl: FloatPoint, to: FloatPoint) {
      let from = self.last();

      for i in 1..self.steps {
         let t2 = i as f64 / self.steps as f64;
         let t1 = 1. - t2;

         let u = on_segment(from, ctrl, t1, t2);
         let v = on_segment(ctrl, to, t1, t2);

         let f = on_segment(u, v, t1, t2);

         self.push(f);
      }

      self.push(to);
   }

   #[inline]
   pub fn cubic_to(&mut self, ctrl1: FloatPoint, ctrl2: FloatPoint, to: FloatPoint) {
      let from = self.last();

      for i in 1..self.steps {
         let t2 = i as f64 / self.steps as f64;
         let t1 = 1. - t2;

         let u = on_segment(from, ctrl1, t1, t2);
         let v = on_segment(ctrl1, ctrl2, t1, t2);
         let w = on_segment(ctrl2, to, t1, t2);

         let m = on_segment(u, v, t1, t2);
         let n = on_segment(v, w, t1, t2);

         let f = on_segment(m, n, t1, t2);

         self.push(f);
      }

      self.push(to);
   }

   #[inline]
   fn push(&mut self, point: FloatPoint) {
      if let Some(last) = self.points.last_mut() {
         last.push(point);
      }
   }

   #[inline]
   fn last(&mut self) -> FloatPoint {
      *self.points.last().unwrap().last().unwrap()
   }

   #[inline]
   fn order_points(&self, offset: f64) -> Vec<Vec<FloatPoint>> {
      let mut outer = Vec::new();

      for contour in &self.points {
         let mut inner = Vec::new();

         for point in contour {
            let x = point.x + offset;
            let y = -point.y;

            inner.push(FloatPoint::new(x, y));
         }

         outer.push(inner);
      }

      outer
   }
}


#[inline]
fn on_segment(pt1: FloatPoint, pt2: FloatPoint, t1: f64, t2: f64) -> FloatPoint {
   let x = t1 * pt1.x + t2 * pt2.x;
   let y = t1 * pt1.y + t2 * pt2.y;

   FloatPoint::new(x, y)
}

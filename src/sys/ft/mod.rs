pub mod ffi;

use std::os::raw::{c_void, c_int};

use std::ptr;
use std::ffi::CString;

use data::FloatPoint;
use geom::point::Point;

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

   pub fn load_face(&self, name: &str) -> Face {
      let cname = CString::new(name).unwrap();

      let mut ft_face: ffi::FT_Face = ptr::null_mut();

      unsafe {
         ffi::FT_New_Face(self.ft_lib, cname.as_ptr() as *const _, 0, &mut ft_face);
      }

      Face::new(ft_face)
   }
}

impl Drop for FreeType {
   fn drop (&mut self) {
      unsafe {
         ffi::FT_Done_FreeType(self.ft_lib);
      }
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
      if let Some(list) = self.points.last() {
         if let Some(last) = list.last() {
            return *last;
         }
      }

      panic!("No points");
   }

   #[inline]
   fn as_int_points(&self) -> Vec<Vec<Point>> {
      let mut outer = Vec::new();

      for contour in &self.points {
         let mut inner = Vec::new();

         for point in contour {
            let x = point.x.round() as i64;
            let y = -(point.y.round() as i64);

            inner.push(Point::new(x, y));
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


pub struct Face {
   pub ft_face: ffi::FT_Face
}

impl Face {
   pub fn new(ft_face: ffi::FT_Face) -> Self {
      Face {
         ft_face: ft_face,
      }
   }

   pub fn set_size(&self, height: i64) {
      unsafe {
         ffi::FT_Set_Pixel_Sizes(self.ft_face, 0, height as ffi::FT_UInt);
      }
   }

   pub fn char_points(&self, ch: char, steps: usize) -> Vec<Vec<Point>> {
      let funcs = ffi::FT_Outline_Funcs {
         move_to: Some(move_to),
         line_to: Some(line_to),
         conic_to: Some(conic_to),
         cubic_to: Some(cubic_to),
         shift: 0,
         delta: 0,
      };

      let mut points: Box<CharPoints> = Box::new(CharPoints::new(steps));

      unsafe {
         ffi::FT_Load_Char(self.ft_face, ch as u32, ffi::FT_LOAD_DEFAULT);

         let slot: ffi::FT_GlyphSlot = (*self.ft_face).glyph as ffi::FT_GlyphSlot;

         ffi::FT_Outline_Decompose(
            &mut (*slot).outline,
            &funcs,
            &mut *points as *mut _ as *mut c_void
         );
      }

      points.as_int_points()
   }
}


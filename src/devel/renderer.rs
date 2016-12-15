use std::marker::Send;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::ptr;
use std::usize;
use std::cmp::{min, max};
use std::u32;
use std::mem::replace;

use renderer::Renderer;
use frame::Frame;
use draw::RGB;
use data::{IntPoint, min_max, min_max_by_x};

use super::Scene;


pub const SUBDIVISIONS: i64 = 4;
pub const SUBDIVISIONS_U: usize = SUBDIVISIONS as usize;
pub const SUBDIVISIONS_X2: i64 = SUBDIVISIONS * SUBDIVISIONS;

//pub const ALPHAS: [u16; 10] = [1, 30, 58, 86, 114, 143, 171, 199, 228, 256];
pub const ALPHAS: [u16; 17] = [1, 17, 33, 49, 65, 81, 97, 113, 128, 144, 160, 176, 192, 208, 224, 240, 256];


pub const ZONE_COUNT: i64 = 8;


#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Edge {
   pub p1y: i64,
   pub p2y: i64,
   pub currx: f64,
   pub drawx: i64,
   pub slope: f64,
}

impl Edge {
   #[inline]
   fn new(p1: &IntPoint, p2y: i64, slope: f64) -> Self {
      Edge {
         p1y: p1.y,
         p2y: p2y,
         currx: p1.x as f64,
         drawx: 0,
         slope: slope,
      }
   }

   #[inline]
   fn advance(&mut self) {
      self.drawx = ((2.0 * self.currx + self.slope) / 2.0).round() as i64;
      self.currx += self.slope;
   }
}


struct IntPointPairs<'a> {
   points: &'a [IntPoint],
   index: usize,
   len: usize,
}


impl<'a> IntPointPairs<'a> {
   fn new(points: &'a [IntPoint]) -> Self {
      let len = points.len();
      IntPointPairs {
         points: points,
         index: 0,
         len: len,
      }
   }
}


impl<'a> Iterator for IntPointPairs<'a> {
   type Item = (IntPoint, IntPoint);

   #[inline]
   fn next(&mut self) -> Option<(IntPoint, IntPoint)> {
      if self.index < self.len {
         let i = self.index;
         self.index += 1;

         let j = if i == self.len - 1 {
            0
         } else {
            i+1
         };

         unsafe {
            let first = self.points.get_unchecked(i);
            let second = self.points.get_unchecked(j);

            Some((*first, *second))
         }
      } else {
         None
      }
   }

   #[inline]
   fn size_hint(&self) -> (usize, Option<usize>) {
      let len = self.len - self.index;
      (len, Some(len))
   }
}

#[derive(Clone, Copy, Debug)]
struct ZoneSplitter {
   aliased_height: i64,
   zone_count: i64,
   zone_height: i64,
}

impl ZoneSplitter {
   #[inline]
   fn new(aliased_height: i64, frame_height: i64, zone_count: i64) -> Self {
      let pixel_height = 1 + (frame_height - 1) / zone_count;
      let zone_height = pixel_height * SUBDIVISIONS;
      let zone_count = 1 + (frame_height - 1) / pixel_height;

      ZoneSplitter {
         aliased_height: aliased_height,
         zone_count: zone_count,
         zone_height: zone_height,
      }
   }

   #[inline]
   pub fn zone_lower(&self, y: i64) -> i64 {
      if y < 0 {
         -1
      } else if y >= self.aliased_height {
         self.zone_count
      } else {
         y / self.zone_height
      }
   }

   #[inline]
   pub fn zone_upper(&self, y: i64) -> i64 {
      if y <= 0 {
         -1
      } else if y > self.aliased_height {
         self.zone_count
      } else {
         (y - 1) / self.zone_height
      }
   }

   #[inline]
   pub fn zone_y_end(&self, zone: i64) -> i64 {
      let zone = zone + 1;

      if self.zone_count == zone {
         self.aliased_height
      } else {
         zone * self.zone_height
      }
   }

   #[inline]
   pub fn is_active(&self, zone: i64) -> bool {
      zone >= 0 && zone < self.zone_count
   }
}


fn collect_vertical(
   edges: &mut Vec<Vec<Edge>>,
   vertical: &Vec<(i64, i64)>,
   splitter: &ZoneSplitter,
   vertical_x: i64,
) {
   for &(y1, y2) in vertical {
      let zone1 = splitter.zone_lower(y1);
      let zone2 = splitter.zone_upper(y2);

      let mut t1 = IntPoint::new(vertical_x, y1);

      for zone in zone1..zone2 {
         let y = splitter.zone_y_end(zone);

         add_edge(edges, &splitter, zone, &t1, y, 0_f64);

         t1 = IntPoint::new(vertical_x, y);
      }

      add_edge(edges, &splitter, zone2, &t1, y2, 0_f64);
   }
}


fn add_edge(
   edges: &mut Vec<Vec<Edge>>,
   splitter: &ZoneSplitter,
   zone: i64,
   p1: &IntPoint,
   p2y: i64,
   slope: f64,
) {
   if p1.y == p2y {
      return;
   }

   let edge = Edge::new(&p1, p2y, slope);

   if splitter.is_active(zone) {
      edges[zone as usize].push(edge);
   }
}


fn sort_edges(edges: &mut Vec<Vec<Edge>>) {
   for zone_edges in edges.iter_mut() {
      zone_edges.sort_by(|a, b| {
         a.p1y.cmp(&b.p1y)
      });
   }
}


fn v_intersect(p: IntPoint, slope: f64, x: i64) -> i64 {
   p.y + ((x - p.x) as f64 / slope).round() as i64
}


fn h_intersect(p: IntPoint, slope: f64, y: i64) -> i64 {
   p.x + ((y - p.y) as f64 * slope).round() as i64
}


fn create_edges_vec(zone_count: i64) -> Vec<Vec<Edge>> {
   let mut edges = Vec::new();

   for _ in 0..zone_count {
      edges.push(Vec::new());
   }

   edges
}


fn clean_edges_vec(edges: &mut Vec<Vec<Edge>>) {
   for list in edges.iter_mut() {
      list.clear();
   }
}


fn add_vertical(vertical: &mut Vec<(i64, i64)>, v1: i64, v2: i64) {
   if let Some(last) = vertical.last_mut() {
      if last.1 == v1 {
         last.1 = v2;
         return;
      } else if last.0 == v2 {
         last.0 = v1;
         return;
      }
   }

   vertical.push((v1, v2));
}


fn build_edges(
   mut edges: &mut Vec<Vec<Edge>>,
   contours: &Vec<Vec<IntPoint>>,
   left_vertical: &mut Vec<(i64, i64)>,
   right_vertical: &mut Vec<(i64, i64)>,
   aliased_width: i64,
   splitter: &ZoneSplitter
) {
   left_vertical.clear();
   right_vertical.clear();

   for points in contours.iter() {
      for (p1, p2) in IntPointPairs::new(points) {
         let (p1, p2) = min_max(p1, p2);

         let (mut q1, mut q2) = min_max_by_x(p1, p2);

         if q2.x <= 0 {
            add_vertical(left_vertical, p1.y, p2.y);
            continue;
         }

         if q1.x >= aliased_width {
            add_vertical(right_vertical, p1.y, p2.y);
            continue;
         }

         let dx = p2.x - p1.x;
         let dy = p2.y - p1.y;

         let slope = dx as f64 / dy as f64;

         if q1.x < 0 {
            let qy = v_intersect(p1, slope, 0);

            let (min_y, max_y) = min_max(qy, q1.y);
            add_vertical(left_vertical, min_y, max_y);

            q1 = IntPoint::new(0, qy);
         }

         if q2.x > aliased_width {
            let qy = v_intersect(p1, slope, aliased_width);

            let (min_y, max_y) = min_max(qy, q2.y);
            add_vertical(right_vertical, min_y, max_y);

            q2 = IntPoint::new(aliased_width, qy);
         }

         let (q1, q2) = min_max(q1, q2);

         let zone1 = splitter.zone_lower(q1.y);
         let zone2 = splitter.zone_upper(q2.y);

         let mut t1 = q1;

         for zone in zone1..zone2 {
            let y = splitter.zone_y_end(zone);
            let x = h_intersect(p1, slope, y);

            add_edge(&mut edges, &splitter, zone, &t1, y, slope);

            t1 = IntPoint::new(x, y);
         }

         add_edge(&mut edges, &splitter, zone2, &t1, q2.y, slope);
      }
   }

   collect_vertical(&mut edges, &left_vertical, &splitter, 0);

   collect_vertical(&mut edges, &right_vertical, &splitter, aliased_width);
}


fn move_from_start(zone_edges: &mut Vec<Edge>, start: usize, offset: usize, end: usize) {
   let count = end.wrapping_sub(start).wrapping_sub(offset);

   unsafe {
      let src_ptr = zone_edges.as_mut_ptr().offset(start as isize);
      let dst_ptr = src_ptr.offset(offset as isize);
      ptr::copy(src_ptr, dst_ptr, count);
   }
}


#[inline]
fn aliased_alpha(mut ptr: *const u8, next_row_offset: isize) -> u16 {
   let mut sum: u8 = 0;

   for _ in 0..SUBDIVISIONS_U {
      for _ in 0..SUBDIVISIONS_U {
         unsafe {
            sum = sum.wrapping_add(*ptr);
            ptr = ptr.offset(1);
         }
      }

      unsafe {
         ptr = ptr.offset(next_row_offset);
      }
   }

   // (255 * sum as u16 / SUBDIVISIONS_X2 as u16) + 1
   unsafe {
      *ALPHAS.get_unchecked(sum as usize)
   }
}


#[inline]
unsafe fn blend_alpha_pixel(mut ptr: *mut u8, a: u16, r: u16, g: u16, b: u16) -> *mut u8 {
   if a == 1 {
      return ptr.offset(4);
   }

   *ptr = blend(r, (*ptr as u16).wrapping_add(1), a);

   ptr = ptr.offset(1);

   *ptr = blend(g, (*ptr as u16).wrapping_add(1), a);

   ptr = ptr.offset(1);

   *ptr = blend(b, (*ptr as u16).wrapping_add(1), a);

   ptr = ptr.offset(1);

   *ptr = blend(256_u16, (*ptr as u16).wrapping_add(1), a);

   ptr.offset(1)
}


#[inline]
fn blend(v1: u16, v2: u16, a1: u16) -> u8 {
   // (((v1 * a1 + v2 * (256 - a1)) >> 8) as u8) - 1

   (v1.wrapping_mul(a1).wrapping_add(
      v2.wrapping_mul(
         256_u16.wrapping_sub(a1)
      )
   ).wrapping_shr(8) as u8).wrapping_sub(1)
}

/*

#[inline]
fn write_to_surface(
   mut src_ptr: *const u8,
   mut dst_ptr: *mut u8,
   width: usize,
   height: usize,
   y: i64,
   y_end: i64
) {
   let width = width as isize;
   let height = height as isize;
   let y = y as isize;
   let y_end = y_end as isize;
   let bytes_width = width.wrapping_mul(4);

   unsafe {
      src_ptr = src_ptr.offset(y.wrapping_mul(bytes_width));
      dst_ptr = dst_ptr.offset(height.wrapping_sub(y).
         wrapping_sub(1).wrapping_mul(bytes_width));

      for _ in 0..y_end.wrapping_sub(y) {
         ptr::copy_nonoverlapping(src_ptr, dst_ptr, bytes_width as usize);

         src_ptr = src_ptr.offset(bytes_width);
         dst_ptr = dst_ptr.offset(-bytes_width);
      }
   }
}

*/

#[inline]
fn write_to_surface(
   mut src_ptr: *const u8,
   mut dst_ptr: *mut u8,
   width: usize,
   y: i64,
   y_end: i64
) {
   let width = width as isize;
   let y = y as isize;
   let y_end = y_end as isize;
   let bytes_width = width.wrapping_mul(4);
   let offset = y.wrapping_mul(bytes_width);

   unsafe {
      src_ptr = src_ptr.offset(offset);
      dst_ptr = dst_ptr.offset(offset);

      ptr::copy_nonoverlapping(
         src_ptr,
         dst_ptr,
         y_end.wrapping_sub(y).wrapping_mul(bytes_width) as usize
      );
   }
}


#[inline]
fn supersample(
   ptr: *mut u8,
   aliased_ptr: *const u8,
   start: usize,
   start_max: usize,
   end: usize,
   end_min: usize,
   y: i64,
   frame_width: usize,
   color: RGB,
) {
   let x_start = start.wrapping_div(SUBDIVISIONS_U);
   let x_start_max = start_max.wrapping_add(SUBDIVISIONS_U).
      wrapping_sub(1).wrapping_div(SUBDIVISIONS_U);

   let x_end = end.wrapping_add(SUBDIVISIONS_U).wrapping_sub(1).wrapping_div(SUBDIVISIONS_U);
   let x_end_min = end_min.wrapping_div(SUBDIVISIONS_U);

   let y = y.wrapping_div(SUBDIVISIONS).wrapping_sub(1) as usize;

   let mut color_u32: u32 = 255_u32;
   color_u32 |= (color.b as u32).wrapping_shl(8_u32);
   color_u32 |= (color.g as u32).wrapping_shl(16_u32);
   color_u32 |= (color.r as u32).wrapping_shl(24_u32);
   color_u32 = u32::from_be(color_u32);

   let r = (color.r as u16).wrapping_add(1);
   let g = (color.g as u16).wrapping_add(1);
   let b = (color.b as u16).wrapping_add(1);

   // (frame width - 1) * SUBDIVISIONS_U
   let next_row_offset = frame_width.wrapping_sub(1).wrapping_mul(SUBDIVISIONS_U) as isize;

   let mut ptr = unsafe {
      ptr.offset(
         y.wrapping_mul(frame_width).wrapping_add(x_start).wrapping_mul(4) as isize
      )
   };

   if x_start_max < x_end_min {
      let src_ptr = unsafe {
         aliased_ptr.offset(
            (x_start.wrapping_mul(SUBDIVISIONS_U)) as isize
         )
      };

      ptr = blend_alpha_stripe(
         ptr, src_ptr, x_start_max - x_start, next_row_offset, r, g, b
      );

      let mut ptr = ptr as *mut u32;

      for _ in 0..x_end_min.wrapping_sub(x_start_max) {
         unsafe {
            *ptr = color_u32;
            ptr = ptr.offset(1);
         }
      }
      let ptr = ptr as *mut u8;

      let src_ptr = unsafe {
         aliased_ptr.offset(
            (x_end_min.wrapping_mul(SUBDIVISIONS_U)) as isize
         )
      };

      blend_alpha_stripe(
         ptr, src_ptr, x_end - x_end_min, next_row_offset, r, g, b
      );
   } else {
      let src_ptr = unsafe {
         aliased_ptr.offset(
            (x_start.wrapping_mul(SUBDIVISIONS_U)) as isize
         )
      };

      blend_alpha_stripe(
         ptr, src_ptr, x_end - x_start, next_row_offset, r, g, b
      );
   }
}


#[inline]
fn blend_alpha_stripe(
   mut dst_ptr: *mut u8,
   mut src_ptr: *const u8,
   len: usize,
   next_row_offset: isize,
   r: u16,
   g: u16,
   b: u16,
) -> *mut u8 {
   unsafe {
      for _ in 0..len {
         let a = aliased_alpha(src_ptr, next_row_offset);
         dst_ptr = blend_alpha_pixel(dst_ptr, a, r, g, b);
         src_ptr = src_ptr.offset(SUBDIVISIONS as isize);
      }
   }

   dst_ptr
}


fn rasterize_edges(
   mut zone_edges: &mut Vec<Edge>,
   color: RGB,
   aliased_ptr: *mut u8,
   ptr: *mut u8,
   y_end: i64,
   frame_width: usize,
) {
   let mut active_start = 0;
   let mut active_end = 0;
   let mut y = 0;

   let mut aliased_start = usize::MAX;
   let mut aliased_start_max = usize::MIN;
   let mut aliased_end = usize::MIN;
   let mut aliased_end_min = usize::MAX;

   let aliased_width = frame_width * SUBDIVISIONS_U;

   let zone_edges_len = zone_edges.len();

   loop {
      let mut i = active_start;
      let mut count = 0;

      // Move away passed edges
      loop {
         if i == active_end {
            if count > 0 {
               move_from_start(&mut zone_edges, active_start, count, i);
               active_start = active_start.wrapping_add(count);
            }
            break;
         }

         if unsafe { zone_edges.get_unchecked(i).p2y == y } {
            count = count.wrapping_add(1);
         } else if count > 0 {
            move_from_start(&mut zone_edges, active_start, count, i);
            active_start = active_start.wrapping_add(count);
            count = 0;
         }

         i = i.wrapping_add(1);
      }

      if active_start == active_end {
         if zone_edges_len == active_end {
            break;
         }

         y = unsafe { zone_edges.get_unchecked(active_end).p1y };

         active_end = active_end.wrapping_add(1);
      }

      loop {
         if zone_edges_len == active_end {
            break;
         }

         if unsafe { zone_edges.get_unchecked(active_end).p1y != y } {
            break;
         }

         active_end = active_end.wrapping_add(1);
      }

      for i in active_start..active_end {
         unsafe { zone_edges.get_unchecked_mut(i).advance() };
      }

      zone_edges[active_start..active_end].sort_by(|a, b| {
         a.drawx.cmp(&b.drawx)
      });

      let xs = &zone_edges[active_start..active_end];

      for i in 0..xs.len().wrapping_div(2) {
         let first = i.wrapping_mul(2);
         let left_x = unsafe { xs.get_unchecked(first).drawx as usize };
         let right_x = unsafe { xs.get_unchecked(first.wrapping_add(1)).drawx as usize };

         aliased_start = min(left_x, aliased_start);
         aliased_start_max = max(left_x, aliased_start_max);
         aliased_end = max(right_x, aliased_end);
         aliased_end_min = min(right_x, aliased_end_min);

         let len = right_x.wrapping_sub(left_x);
         let start = ((y % SUBDIVISIONS) as usize).wrapping_mul(aliased_width);

         unsafe {
            let vp = aliased_ptr.offset((start.wrapping_add(left_x)) as isize);
            ptr::write_bytes(vp, 1, len);
         }
      }

      y = y.wrapping_add(1);

      if y % SUBDIVISIONS == 0 {
         supersample(
            ptr,
            aliased_ptr,
            aliased_start,
            aliased_start_max,
            aliased_end,
            aliased_end_min,
            y,
            frame_width,
            color
         );

         let len = aliased_width.wrapping_mul(SUBDIVISIONS_U);
         unsafe {
            ptr::write_bytes(aliased_ptr, 0, len);
         }

         aliased_start = usize::MAX;
         aliased_start_max = usize::MIN;
         aliased_end = usize::MIN;
         aliased_end_min = usize::MAX;

         if y == y_end {
            break;
         }
      }
   }
}


fn thread_rasterize(thread_rx: Receiver<ThreadInput>, thread_tx: Sender<usize>) {
   let mut aliased = Vec::new();

   loop {
      let input = thread_rx.recv().unwrap();

      match input {
         ThreadInput::Render(input) => {
            let RenderInput {
               mut zone_polys,
               layer_ptr,
               frame_ptr,
               y,
               y_end,
               frame_width,
            } = input;

            let aliased_len = (frame_width as usize) * (SUBDIVISIONS_X2 as usize);
            if aliased.len() != aliased_len {
               aliased.resize(aliased_len, 0);
            }

            let aliased_ptr = aliased.as_mut_ptr();

            for &mut (ref mut zone_edges, color) in zone_polys.iter_mut() {
               rasterize_edges(
                  zone_edges,
                  color,
                  aliased_ptr,
                  layer_ptr,
                  y_end,
                  frame_width,
               );
            }

            let y = y.wrapping_div(SUBDIVISIONS);
            let y_end = y_end.wrapping_div(SUBDIVISIONS);
            let bytes_width = (frame_width).wrapping_mul(4);

            write_to_surface(
               layer_ptr,
               frame_ptr,
               frame_width,
               y,
               y_end,
            );

            unsafe {
               ptr::write_bytes(
                  layer_ptr.offset((y as isize).wrapping_mul(bytes_width as isize)),
                  0,
                  ((y_end - y) as usize).wrapping_mul(bytes_width),
               );
            }
         },
      }

      thread_tx.send(0).unwrap();
   }
}


struct RenderInput {
   zone_polys: Vec<(Vec<Edge>, RGB)>,
   layer_ptr: *mut u8,
   frame_ptr: *mut u8,
   y: i64,
   y_end: i64,
   frame_width: usize,
}

unsafe impl Send for RenderInput {
}


enum ThreadInput {
   Render(RenderInput),
}


struct ThreadChannels {
   render_tx_vec: Vec<Sender<ThreadInput>>,
   result_rx: Receiver<usize>,
}


pub struct DevelRenderer {
   scene: Scene,
   layer: Vec<u8>,
   left_vertical: Vec<(i64, i64)>,
   right_vertical: Vec<(i64, i64)>,
   edges: Vec<Vec<Edge>>,
   channels: ThreadChannels,
}


impl DevelRenderer {
   #[inline]
   pub fn new(scene: Scene) -> Self {
      let channels = Self::start_render_threads();

      DevelRenderer {
         scene: scene,
         layer: Vec::new(),
         left_vertical: Vec::new(),
         right_vertical: Vec::new(),
         edges: create_edges_vec(ZONE_COUNT),
         channels: channels,
      }
   }

   #[inline]
   pub fn set_scene(&mut self, scene: Scene) {
      self.scene = scene;
   }

   #[inline]
   fn start_render_threads() -> ThreadChannels {
      let (result_tx, result_rx) = channel();

      let mut render_tx_vec = Vec::new();

      for _ in 0..ZONE_COUNT as usize {
         let (render_tx, render_rx) = channel();

         render_tx_vec.push(render_tx);

         let thread_tx = result_tx.clone();

         thread::spawn(move|| {
            thread_rasterize(render_rx, thread_tx);
         });
      }

      ThreadChannels {
         render_tx_vec: render_tx_vec,
         result_rx: result_rx,
      }
   }

   #[inline]
   fn check_resize(&mut self, width: u32, height: u32) -> bool {
      let mut resized = false;

      let layer_len = 4 * (width as usize * height as usize);
      if self.layer.len() != layer_len {
         self.layer.resize(layer_len, 0);
         resized = true;
      }

      resized
   }

   #[inline]
   fn wait_threads(&self, splitter: &ZoneSplitter) {
      for _ in 0..splitter.zone_count {
         self.channels.result_rx.recv().unwrap();
      }
   }
}

impl Renderer for DevelRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      let _ = self.check_resize(width, height);
   }

   fn render(&mut self, frame: &mut Frame) {
      let _ = self.check_resize(frame.width, frame.height);

      let frame_width = frame.width as usize;
      let frame_height = frame.height as usize;

      let aliased_width = frame.width as i64 * SUBDIVISIONS;
      let aliased_height = frame.height as i64 * SUBDIVISIONS;

      let splitter = ZoneSplitter::new(aliased_height, frame_height as i64, ZONE_COUNT);

      let mut all_zones_polys = Vec::new();

      for _ in 0..splitter.zone_count {
         all_zones_polys.push(Vec::new());
      }

      for poly in self.scene.polys.iter() {
         clean_edges_vec(&mut self.edges);

         build_edges(
            &mut self.edges,
            &poly.contours,
            &mut self.left_vertical,
            &mut self.right_vertical,
            aliased_width,
            &splitter
         );

         sort_edges(&mut self.edges);

         for zone in 0..splitter.zone_count as usize {
            unsafe {
               if self.edges.get_unchecked(zone).len() > 0 {
                  let zone_edges = replace(self.edges.get_unchecked_mut(zone), Vec::new());

                  all_zones_polys.get_unchecked_mut(zone).push((zone_edges, poly.color));
               }
            }
         }
      }

      let layer_ptr = self.layer.as_mut_ptr();
      let mut y = 0;

      let frame_ptr = frame.ptr_mut();

      for zone in 0..splitter.zone_count as usize {
         let render_tx = unsafe {
            self.channels.render_tx_vec.get_unchecked(zone)
         };

         let zone_polys = unsafe {
            replace(all_zones_polys.get_unchecked_mut(zone), Vec::new())
         };

         let y_end = splitter.zone_y_end(zone as i64);

         render_tx.send(
            ThreadInput::Render(
               RenderInput {
                  zone_polys: zone_polys,
                  layer_ptr: layer_ptr,
                  frame_ptr: frame_ptr,
                  y: y,
                  y_end: y_end,
                  frame_width: frame_width,
               }
            )
         ).unwrap();

         y = y_end;
      }

      self.wait_threads(&splitter);
   }
}

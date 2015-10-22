use std::iter::repeat;

use std::ops::{Index, RangeFull};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Ring<T> {
   v: Vec<T>,
   start: usize,
   end: usize,
}

impl<T> Ring<T> where T: Default + Clone + Debug {
   #[inline]
   pub fn new(capacity: usize) -> Self {
      let v = fill_default::<T>(capacity);

      Ring { v: v, start: 0, end: 0 }
   }

   #[inline]
   pub fn len(&self) -> usize {
      self.end - self.start
   }

   #[inline]
   pub fn last(&self) -> Option<&T> {
      if self.start == self.end {
         None
      } else {
         Some(unsafe {
            self.v.get_unchecked(self.end - 1)
         })
      }
   }

   #[inline]
   pub fn start(&self) -> usize {
      self.start
   }

   #[inline]
   pub fn end(&self) -> usize {
      self.end
   }

   #[inline]
   pub fn consume_at(&mut self, marker: usize) {
      self.start = marker;
   }

   #[inline]
   pub fn consume(&mut self) {
      self.start = self.end;
   }

   #[inline]
   pub fn reserve(&mut self, additional: usize) {
      self.v.reserve(additional);
   }

   #[inline]
   pub fn rewind(&mut self, extra: usize) {
      if extra > (self.v.len() - self.end) {
         if self.start != self.end {
            for i in 0..(self.end - self.start) {
               self.v[i] = self.v[self.start + i].clone();
            }
         }

         self.end -= self.start;
         self.start = 0;
      }
   }

   #[inline]
   pub fn push(&mut self, value: T) {
      self.v[self.end] = value;
      self.end += 1;
   }

   #[inline]
   pub fn next_index(&self, index: usize) -> usize {
      let index = index + 1;
      if index == self.end {
         self.start
      } else {
         index
      }
   }

   #[inline]
   pub fn prev_index(&self, index: usize) -> usize {
      if index == self.start {
         self.end - 1
      } else {
         index - 1
      }
   }

   #[inline]
   pub fn clear(&mut self) {
      self.start = 0;
      self.end = 0;
   }
}

impl<T> Index<usize> for Ring<T> {
   type Output = T;

   #[inline]
   fn index(&self, index: usize) -> &T {
      &self.v[index]
   }
}

impl<T> Index<RangeFull> for Ring<T> {
   type Output = [T];

   #[inline]
   fn index(&self, _index: RangeFull) -> &[T] {
      &self.v[self.start..self.end]
   }
}

pub fn fill_default<T>(capacity: usize) -> Vec<T> where T: Default + Clone {
   repeat(T::default()).take(capacity).collect()
}

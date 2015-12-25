use super::rasterizer::Rasterizer;
use super::edge::Edge;


pub trait RasterizerPool {
   fn check_upper_initial_pool(&self);
   fn check_upper_pool(&self);

   fn check_lower_initial_pool(&self);
   fn check_lower_pool(&self);

   fn check_final_pool(&self);

   fn check_pool_poly(&self, poly_index: usize, pool: &Vec<Edge>, pool_lens: &Vec<usize>);

   fn check_upper_bounds(&self, y_slice: i64);
   fn check_lower_initial_bounds(&self, y_slice: i64);
   fn check_lower_bounds(&self, x_slice: i64);
   fn check_final_bounds(&self, x_slice: i64);
}

impl RasterizerPool for Rasterizer {
   fn check_upper_initial_pool(&self) {
      for poly_index in 0..self.polys_len {
         self.check_pool_poly(poly_index, &self.upper_edges, &self.upper_edges_len);
      }
   }

   fn check_upper_pool(&self) {
      for active_index in self.upper_active_start..self.upper_active_end {
         let poly_index = self.upper_active[active_index];
         self.check_pool_poly(poly_index, &self.upper_edges, &self.upper_edges_len);
      }
   }

   fn check_lower_initial_pool(&self) {
      for active_index in 0..self.lower_active_full {
         let poly_index = self.lower_active[active_index];
         self.check_pool_poly(poly_index, &self.lower_edges, &self.lower_edges_len);
      }
   }

   fn check_lower_pool(&self) {
      for active_index in self.lower_active_start..self.lower_active_end {
         let poly_index = self.lower_active[active_index];
         self.check_pool_poly(poly_index, &self.lower_edges, &self.lower_edges_len);
      }
   }

   fn check_final_pool(&self) {
      for active_index in 0..self.final_active_full {
         let poly_index = self.final_active[active_index];
         self.check_pool_poly(poly_index, &self.final_edges, &self.final_edges_len);
      }
   }

   fn check_pool_poly(&self, poly_index: usize, pool: &Vec<Edge>, pool_lens: &Vec<usize>) {
      let poly_start = self.poly_to_pool[poly_index];
      let poly_len = pool_lens[poly_index];
      let poly_end = poly_start + poly_len;

      if poly_len < 3 {
         panic!("Insufficient edge count: {}", poly_len);
      }

      let mut p2_prev = pool[poly_end - 1].p2;
      for edge_index in poly_start..poly_end {
         let edge = pool[edge_index];

         if edge.edge_type.reversed() != (edge.p1 > edge.p2) {
            panic!("Wrong edge points ordering");
         }

         if edge.p1 != p2_prev {
            panic!(
               "Unconnected poly [{}] i {} start {} end {} ({}, {}) / ({}, {})",
               poly_index, edge_index, poly_start, poly_end, p2_prev.x, p2_prev.y, edge.p1.x, edge.p1.y
            );
         }

         p2_prev = edge.p2;
      }
   }

   fn check_upper_bounds(&self, y_slice: i64) {
      for i in self.upper_active_start..self.upper_active_end {
         let poly_index = self.upper_active[i];

         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.upper_edges_len[poly_index];

         for edge_index in poly_start..poly_end {
            let ref edge = self.upper_edges[edge_index];

            if edge.p1.y < y_slice {
               panic!(
                  "Upper polygon below slice point - Poly: {}, Edge / Slice Y: {} {}",
                  poly_index, edge.p1.y, y_slice
               );
            }
         }
      }
   }

   fn check_lower_bounds(&self, x_slice: i64) {
      for i in self.lower_active_start..self.lower_active_end {
         let poly_index = self.lower_active[i];

         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.lower_edges_len[poly_index];

         for edge_index in poly_start..poly_end {
            let ref edge = self.lower_edges[edge_index];

            if edge.p1.x < x_slice {
               panic!(
                  "Lower polygon to the left slice point - Poly: {}, Edge / Slice X: {} {}",
                  poly_index, edge.p1.x, x_slice
               );
            }
         }
      }
   }

   fn check_lower_initial_bounds(&self, y_slice: i64) {
      for active_index in 0..self.lower_active_full {
         let poly_index = self.lower_active[active_index];

         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.lower_edges_len[poly_index];

         for edge_i in poly_start..poly_end {
            let ref edge = self.lower_edges[edge_i];

            if edge.p1.y > y_slice  {
               panic!(
                  "Lower polygon above slice point - Poly: {}, Edge / Slice Y: {} {}",
                  poly_index, edge.p1.y, y_slice
               );
            }
         }
      }
   }

   fn check_final_bounds(&self, x_slice: i64) {
      for active_index in 0..self.final_active_full {
         let poly_index = self.final_active[active_index];

         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.final_edges_len[poly_index];

         for edge_i in poly_start..poly_end {
            let ref edge = self.final_edges[edge_i];

            if edge.p1.x > x_slice  {
               panic!(
                  "Final polygon to the right of slice point - Poly: {}, Edge / Slice X: {} {}",
                  poly_index, edge.p1.x, x_slice
               );
            }
         }
      }
   }
}


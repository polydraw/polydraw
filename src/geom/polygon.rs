use std::cmp::Ordering;

use super::point::Point;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Polygon {
   pub points: Vec<Point>
}

impl Polygon {
   pub fn new(points: Vec<Point>) -> Self {
      Polygon {
         points: points
      }
   }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
pub struct Edge {
   pub p1: Point,
   pub p2: Point,
   pub dx: i64,
   pub dy: i64,
}

impl Edge {
   fn new(a1: &Point, a2: &Point) -> Self {
      let (p1, p2) = if a1 < a2 {
         (a1, a2)
      } else {
         (a2, a1)
      };

      Edge {
         p1: *p1,
         p2: *p2,
         dx: p2.x - p1.x,
         dy: p2.y - p1.y,
      }
   }
}

impl PartialOrd for Edge {
   fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
      Some(
         match self.p1.cmp(&other.p1) {
            Ordering::Greater => {
               Ordering::Greater
            },
            Ordering::Less => {
               Ordering::Less
            },
            Ordering::Equal => {
               if self.dy == 0 {
                  if other.dy == 0 {
                     self.dx.cmp(&other.dx)
                  } else {
                     if self.dx > 0 {
                        Ordering::Greater
                     } else {
                        Ordering::Less
                     }
                  }
               } else if other.dy == 0 {
                  if other.dx > 0 {
                     Ordering::Less
                  } else {
                     Ordering::Greater
                  }
               } else {
                  match (self.dx * other.dy).cmp(&(self.dy * other.dx)) {
                     Ordering::Greater => {
                        Ordering::Greater
                     },
                     Ordering::Less => {
                        Ordering::Less
                     },
                     Ordering::Equal => {
                        self.p2.cmp(&other.p2)
                     }
                  }
               }
            }
         }
      )
   }
}

fn cmp_edges_by_y(first: &Edge, second: &Edge) -> Ordering {
   first.p1.y.cmp(&second.p1.y)
}

pub fn to_scanline_edges(points: &Vec<Point>) -> Vec<Edge> {
   let mut edges = Vec::new();

   let mut p1 = points.last().unwrap();

   for p2 in points.iter() {
      let edge = Edge::new(p1, p2);

      if edge.dy != 0 {
         edges.push(edge);
      }

      p1 = p2
   }

   edges.sort_by(cmp_edges_by_y);

   edges
}

#[inline]
pub fn orientation(a: &Point, b: &Point, c: &Point) -> i64 {
   (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn double_polygon_area(start: usize, points: &Vec<Point>) -> i64 {
   let mut p1 = points.last().unwrap();

   let mut area = 0;

   for p2 in points[start..].iter() {
      area += p1.x * p2.y - p1.y * p2.x;

      p1 = p2;
   }

   area.abs()
}

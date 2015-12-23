use geom::point::Point;
use draw::RGB;

use super::segment::Segment;
use super::circle::Circle;
use super::poly::Poly;
use super::edge::EdgeSrc;


pub struct Scene {
   pub points: Vec<Point>,
   pub segments: Vec<Segment>,
   pub circles: Vec<Circle>,
   pub edges: Vec<EdgeSrc>,
   pub polys: Vec<Poly>,
   pub colors: Vec<RGB>,
}

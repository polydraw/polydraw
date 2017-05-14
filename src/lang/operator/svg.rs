use std::f64::consts::PI;

use devel::SUBDIVISIONS;
use data::{Empty, FloatPoint};

use lang::variant::Variant;
use lang::execute::Executor;
use lang::compiler::FnRef;


#[derive(PartialEq, Clone, Debug)]
pub enum CommandType {
   MoveTo,
   MoveToRelative,
   LineTo,
   LineToRelative,
   Horizontal,
   HorizontalRelative,
   Vertical,
   VerticalRelative,
   CurveTo,
   CurveToRelative,
   SmoothCurveTo,
   SmoothCurveToRelative,
   Quadratic,
   QuadraticRelative,
   SmoothQuadratic,
   SmoothQuadraticRelative,
   EllipticalArc,
   EllipticalArcRelative,
   ClosePath,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Token {
   Command(CommandType),
   Float(f64),
}


#[derive(Clone, Debug)]
pub enum Command {
   MoveTo {
      p: FloatPoint,
   },
   LineTo {
      p: FloatPoint,
   },
   Horizontal {
      x: f64,
   },
   Vertical {
      y: f64,
   },
   CurveTo {
      p1: FloatPoint,
      p2: FloatPoint,
      p: FloatPoint,
   },
   SmoothCurveTo {
      p2: FloatPoint,
      p: FloatPoint,
   },
   Quadratic {
      p1: FloatPoint,
      p: FloatPoint,
   },
   SmoothQuadratic {
      p: FloatPoint,
   },
   EllipticalArc {
      rx: f64,
      ry: f64,
      rotation: f64,
      large: bool,
      sweep: bool,
      p: FloatPoint,
   },
   ClosePath,
}

pub type TokenResult = Option<(Token, usize)>;


pub fn tokenize_svg_path(string: &str) -> Option<Vec<Token>> {
   let mut tokens = Vec::new();

   let chars_vec: Vec<char> = string.chars().collect();

   let mut source: &[char] = &chars_vec;

   loop {
      if source.len() == 0 {
         break;
      }

      if let Some((token, taken)) = single_token(source) {
         tokens.push(token);

         source = &source[taken..];
      } else {
         source = &source[1..];
      }
   }

   if tokens.len() == 0 {
      None
   } else {
      Some(tokens)
   }
}


fn single_token(source: &[char]) -> TokenResult {
   if let Some(result) = extract_number(source) {
      Some(result)
   } else if let Some(result) = extract_command(source) {
      Some(result)
   } else {
      None
   }
}


fn extract_number(source: &[char]) -> TokenResult {
   let full_len = source.len();

   let mut end = 0;

   let positive = match source[0] {
      '-' => {
         end += 1;
         false
      },
      '+' => {
         end += 1;
         true
      },
      _ => {
         true
      },
   };


   let mut integral = 0;

   let mut source = &source[end..];
   end = 0;

   for ch in source.iter() {
      if let Some(digit) = to_digit(*ch) {
         integral = 10 * integral + digit;
         end += 1;
      } else {
         break;
      }
   }

   let no_digits = end == 0;

   source = &source[end..];


   let mut fractional = 0;
   let mut divisor = 1;

   if let Some(&'.') = source.first() {
      source = &source[1..];

      end = 0;

      for ch in source.iter() {
         if let Some(digit) = to_digit(*ch) {
            fractional = 10 * fractional + digit;
            divisor = 10 * divisor;

            end += 1;
         } else {
            break;
         }
      }

      if end == 0 && no_digits {
         return None;
      }

      source = &source[end..];
   } else if no_digits {
      return None;
   }


   let mut value = integral as f64 + fractional as f64 / divisor as f64;

   if !positive {
      value = -value
   }


   match source.first() {
      Some(&'E') | Some(&'e') => {
         source = &source[1..];

         let mut end = 0;

         let positive = match source.first() {
            None => {
               return None;
            },
            Some(&'-') => {
               end += 1;
               false
            },
            Some(&'+') => {
               end += 1;
               true
            },
            Some(_) => {
               true
            },
         };

         let mut exponent = 0;

         source = &source[end..];

         end = 0;

         for ch in source.iter() {
            if let Some(digit) = to_digit(*ch) {
               exponent = 10 * exponent + digit as i32;
               end += 1;
            } else {
               break;
            }
         }

         if end == 0 {
            return None;
         }

         source = &source[end..];

         let power = 10_f64.powi(exponent);

         if positive {
            value *= power;
         } else {
            value /= power;
         }
      },
      _ => {}
   }

   Some((
      Token::Float(value),
      full_len - source.len()
   ))
}


fn to_digit(ch: char) -> Option<i64> {
   match ch {
      '0' ... '9' => Some((ch as u32 - '0' as u32) as i64),
      _ =>  None
   }
}


fn extract_command(source: &[char]) -> TokenResult {
   let command_type = match source[0] {
      'M' => CommandType::MoveTo,
      'm' => CommandType::MoveToRelative,
      'L' => CommandType::LineTo,
      'l' => CommandType::LineToRelative,
      'H' => CommandType::Horizontal,
      'h' => CommandType::HorizontalRelative,
      'V' => CommandType::Vertical,
      'v' => CommandType::VerticalRelative,
      'C' => CommandType::CurveTo,
      'c' => CommandType::CurveToRelative,
      'S' => CommandType::SmoothCurveTo,
      's' => CommandType::SmoothCurveToRelative,
      'Q' => CommandType::Quadratic,
      'q' => CommandType::QuadraticRelative,
      'T' => CommandType::SmoothQuadratic,
      't' => CommandType::SmoothQuadraticRelative,
      'A' => CommandType::EllipticalArc,
      'a' => CommandType::EllipticalArcRelative,
      'Z' => CommandType::ClosePath,
      'z' => CommandType::ClosePath,
      _ => return None,
   };

   Some((Token::Command(command_type), 1))
}


fn parse_svg_path(mut tokens: &[Token]) -> Option<Vec<(Command, bool)>> {
   let mut previous = CommandType::ClosePath;

   let mut commands = Vec::new();

   loop {
      if tokens.len() == 0 {
         break;
      }

      if let Ok((command_type, command, absolute, rest)) = parse_next_command(tokens, previous) {
         commands.push((command, absolute));

         tokens = rest;

         previous = command_type;
      } else {
         return None;
      }
   }

   Some(commands)
}


fn parse_next_command(
   mut tokens: &[Token],
   previous_type: CommandType
) -> Result<(CommandType, Command, bool, &[Token]), ()> {
   let mut command_type = previous_type;

   if let Token::Command(ref defined_type) = tokens[0] {
      command_type = defined_type.clone();

      tokens = &tokens[1..];
   }

   let (command, absolute) = match command_type {
      CommandType::MoveTo | CommandType::MoveToRelative => {
         let (p, rest) = try!(get_point(tokens));

         tokens = rest;
         let command = Command::MoveTo {p: p};
         let absolute = command_type == CommandType::MoveTo;
         (command, absolute)
      },
      CommandType::LineTo | CommandType::LineToRelative => {
         let (p, rest) = try!(get_point(tokens));

         tokens = rest;
         let command = Command::LineTo {p: p};
         let absolute = command_type == CommandType::LineTo;
         (command, absolute)
      },
      CommandType::Horizontal | CommandType::HorizontalRelative => {
         let (x, rest) = try!(get_number(tokens));

         tokens = rest;
         let command = Command::Horizontal {x: x};
         let absolute = command_type == CommandType::Horizontal;
         (command, absolute)
      },
      CommandType::Vertical | CommandType::VerticalRelative => {
         let (y, rest) = try!(get_number(tokens));

         tokens = rest;
         let command = Command::Vertical {y: y};
         let absolute = command_type == CommandType::Vertical;
         (command, absolute)
      },
      CommandType::CurveTo | CommandType::CurveToRelative => {
         let (p1, rest) = try!(get_point(tokens));
         let (p2, rest) = try!(get_point(rest));
         let (p, rest) = try!(get_point(rest));

         tokens = rest;
         let command = Command::CurveTo {p1: p1, p2: p2, p: p};
         let absolute = command_type == CommandType::CurveTo;
         (command, absolute)
      },
      CommandType::SmoothCurveTo | CommandType::SmoothCurveToRelative => {
         let (p2, rest) = try!(get_point(tokens));
         let (p, rest) = try!(get_point(rest));

         tokens = rest;
         let command = Command::SmoothCurveTo {p2: p2, p: p};
         let absolute = command_type == CommandType::SmoothCurveTo;
         (command, absolute)
      },
      CommandType::Quadratic | CommandType::QuadraticRelative => {
         let (p1, rest) = try!(get_point(tokens));
         let (p, rest) = try!(get_point(rest));

         tokens = rest;
         let command = Command::Quadratic {p1: p1, p: p};
         let absolute = command_type == CommandType::Quadratic;
         (command, absolute)
      },
      CommandType::SmoothQuadratic | CommandType::SmoothQuadraticRelative => {
         let (p, rest) = try!(get_point(tokens));

         tokens = rest;
         let command = Command::SmoothQuadratic {p: p};
         let absolute = command_type == CommandType::SmoothQuadratic;
         (command, absolute)
      },
      CommandType::EllipticalArc | CommandType::EllipticalArcRelative => {
         let (rx, rest) = try!(get_number(tokens));
         let (ry, rest) = try!(get_number(rest));
         let (rotation, rest) = try!(get_number(rest));
         let (large, rest) = try!(get_number(rest));
         let (sweep, rest) = try!(get_number(rest));
         let (p, rest) = try!(get_point(rest));

         tokens = rest;

         let command = Command::EllipticalArc {
            rx: rx,
            ry: ry,
            rotation: rotation,
            large: large == 1.0,
            sweep: sweep == 1.0,
            p: p,
         };

         let absolute = command_type == CommandType::EllipticalArc;

         (command, absolute)
      },
      CommandType::ClosePath => (Command::ClosePath, true),
   };

   Ok((command_type, command, absolute, tokens))
}


fn get_point(tokens: &[Token]) -> Result<(FloatPoint, &[Token]), ()> {
   if tokens.len() < 2 {
      return Err(());
   }

   let x = match tokens[0] {
      Token::Float(value) => value,
      _ => return Err(()),
   };

   let y = match tokens[1] {
      Token::Float(value) => value,
      _ => return Err(()),
   };

   Ok((FloatPoint::new(x, y), &tokens[2..]))
}


fn get_number(tokens: &[Token]) -> Result<(f64, &[Token]), ()> {
   if tokens.len() == 0 {
      return Err(());
   }

   let value = match tokens[0] {
      Token::Float(value) => value,
      _ => return Err(()),
   };

   Ok((value, &tokens[1..]))
}


fn process_path(commands: Vec<(Command, bool)>, steps: usize) -> Vec<Vec<FloatPoint>> {
   let mut contours = Vec::new();

   let mut active = Vec::new();

   let mut last = FloatPoint::default();

   let mut start = FloatPoint::default();

   let mut last_control = FloatPoint::default();

   for (command, absolute) in commands {
      match command {
         Command::MoveTo {p} => {
            let p = to_absolute(p, last, absolute);

            active.push(p);

            last = p;
            start = p;
            last_control = last;
         },
         Command::CurveTo {p1, p2, p} => {
            let p1 = to_absolute(p1, last, absolute);
            let p2 = to_absolute(p2, last, absolute);
            let p = to_absolute(p, last, absolute);

            cubic_to(
               &mut active,
               last,
               p1,
               p2,
               p,
               steps
            );

            last = p;
            last_control = p2;
         },
         Command::SmoothCurveTo {p2, p} => {
            let p1 = last + (last - last_control);
            let p2 = to_absolute(p2, last, absolute);
            let p = to_absolute(p, last, absolute);

            cubic_to(
               &mut active,
               last,
               p1,
               p2,
               p,
               steps
            );

            last = p;
            last_control = p2;
         },
         Command::Quadratic {p1, p} => {
            let p1 = to_absolute(p1, last, absolute);
            let p = to_absolute(p, last, absolute);

            conic_to(
               &mut active,
               last,
               p1,
               p,
               steps
            );

            last = p;
            last_control = p1;
         },
         Command::SmoothQuadratic {p} => {
            let p1 = last + (last - last_control);
            let p = to_absolute(p, last, absolute);

            conic_to(
               &mut active,
               last,
               p1,
               p,
               steps
            );

            last = p;
            last_control = p1;
         },
         Command::EllipticalArc {rx, ry, rotation, large, sweep, p} => {
            assert!(rx == ry);
            assert!(rotation == 0.0);

            let p = to_absolute(p, last, absolute);

            arc_to(
               &mut active,
               last,
               p,
               rx,
               large,
               sweep,
               steps
            );

            last = p;
            last_control = last;
         },
         Command::LineTo {p} => {
            let p = to_absolute(p, last, absolute);

            active.push(p);

            last = p;
            last_control = last;
         },
         Command::Horizontal {x} => {
            let p = if absolute {
               FloatPoint::new(x, last.y)
            } else {
               FloatPoint::new(last.x + x, last.y)
            };

            active.push(p);

            last = p;
            last_control = last;
         },
         Command::Vertical {y} => {
            let p = if absolute {
               FloatPoint::new(last.x, y)
            } else {
               FloatPoint::new(last.x, last.y + y)
            };

            active.push(p);

            last = p;
            last_control = last;
         },
         Command::ClosePath => {
            if active.len() > 0 {
               contours.push(active);
            }

            active = vec![];
            last = start;
         },
      }
   }

   contours
}


#[inline]
fn to_absolute(p: FloatPoint, last: FloatPoint, absolute: bool) -> FloatPoint {
   if absolute {
      p
   } else {
      p + last
   }
}


fn conic_to(
   contour: &mut Vec<FloatPoint>,
   from: FloatPoint,
   ctrl: FloatPoint,
   to: FloatPoint,
   steps: usize
) {
   for i in 1..steps {
      let t2 = i as f64 / steps as f64;
      let t1 = 1. - t2;

      let u = on_segment(from, ctrl, t1, t2);
      let v = on_segment(ctrl, to, t1, t2);

      let f = on_segment(u, v, t1, t2);

      contour.push(f);
   }

   contour.push(to);
}


fn cubic_to(
   contour: &mut Vec<FloatPoint>,
   from: FloatPoint,
   ctrl1: FloatPoint,
   ctrl2: FloatPoint,
   to: FloatPoint,
   steps: usize
) {
   for i in 1..steps {
      let t2 = i as f64 / steps as f64;
      let t1 = 1. - t2;

      let u = on_segment(from, ctrl1, t1, t2);
      let v = on_segment(ctrl1, ctrl2, t1, t2);
      let w = on_segment(ctrl2, to, t1, t2);

      let m = on_segment(u, v, t1, t2);
      let n = on_segment(v, w, t1, t2);

      let f = on_segment(m, n, t1, t2);

      contour.push(f);
   }

   contour.push(to);
}


#[inline]
fn on_segment(
   pt1: FloatPoint,
   pt2: FloatPoint,
   t1: f64,
   t2: f64
) -> FloatPoint {
   let x = t1 * pt1.x + t2 * pt2.x;
   let y = t1 * pt1.y + t2 * pt2.y;

   FloatPoint::new(x, y)
}


fn arc_to(
   contour: &mut Vec<FloatPoint>,
   from: FloatPoint,
   to: FloatPoint,
   mut radius: f64,
   large: bool,
   sweep: bool,
   steps: usize
) {
   let middle = (from + to) / 2;

   let vx = to.x - from.x;
   let vy = to.y - from.y;

   let len = vx.hypot(vy);

   let half_len = len / 2.;

   if radius < half_len {
      radius = half_len;
   }

   let dist = (radius.powi(2) - half_len.powi(2)).sqrt();

   let vp = FloatPoint::new(-vy / len, vx / len) * dist;

   let center = if sweep {
      middle + vp
   } else {
      middle - vp
   };

   let a1 = angle_of(center, from);
   let a2 = angle_of(center, to);

   let mut delta = a2 - a1;

   if delta > PI {
      delta -= 2. * PI;
   } else if delta < -PI {
      delta += 2. * PI;
   }

   if large {
      if delta > 0. {
         delta -= 2. * PI;
      } else {
         delta += 2. * PI;
      }
   }

   let segments = ((steps as f64) * delta.abs() * 6. / PI) as usize;

   let delta_step = delta / segments as f64;

   for i in 1..segments {
      let angle = a1 + delta_step * i as f64;

      let point = FloatPoint::new(
         radius * angle.cos() + center.x,
         radius * angle.sin() + center.y
      );

      contour.push(point);
   }

   contour.push(to);
}


fn angle_of(center: FloatPoint, point: FloatPoint) -> f64 {
   let dx = point.x - center.x;
   let dy = point.y - center.y;

   let mut angle = (dy / dx).abs().atan();

   if dx < 0. {
      if dy < 0. {
         angle += PI;
      } else {
         angle = PI - angle;
      }
   } else {
      if dy < 0. {
         angle = 2. * PI - angle;
      }
   }

   angle
}


fn to_value_ptr_points(executor: &Executor, contours: Vec<Vec<FloatPoint>>) -> Vec<Variant> {
   let mut result = Vec::new();

   for contour in contours {
      let mut inner = Vec::new();

      for point in contour {
         inner.push(executor.registry.variant(point * SUBDIVISIONS as f64));
      }

      result.push(executor.registry.variant(inner));
   }

   vecval!(executor, result)
}


pub fn svg_path(
   arguments: &[&Variant],
   executor: &Executor,
   _: &FnRef
) -> Vec<Variant> {
   let path = arguments[0].as_ref::<String>();

   if let Some(tokens) = tokenize_svg_path(path) {
      if let Some(commands) = parse_svg_path(&tokens) {
         let contours = process_path(commands, 20);

         to_value_ptr_points(executor, contours)
      } else {
         vecval!(executor, Empty)
      }
   } else {
      vecval!(executor, Empty)
   }
}


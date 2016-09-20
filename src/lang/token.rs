
#[derive(Clone, Debug)]
pub enum Token {
   Name(String),
   Integer(i64),
   Float(f64),
   NewLine,
   Comma,
   Assign,
   Add,
   Subtract,
   Multiply,
   Divide,
   ParenLeft,
   ParenRight,
   BracketLeft,
   BracketRight,
}


pub type TokenResult<'a> = Option<(Token, &'a str)>;


pub fn tokenize(source: &str) -> Vec<Token> {
   let mut tokens = Vec::new();

   let mut source = source;

   loop {
      source = ws(source);

      if let Some((token, rest)) = single_token(source) {
         tokens.push(token);
         source = rest;
      } else {
         break;
      }
   }

   tokens
}

fn ws(source: &str) -> &str {
   for (index, ch) in source.chars().enumerate() {
      if ch != ' ' && ch != '\t' {
         return &source[index..];
      }
   }

   source
}


fn single_token(source: &str) -> TokenResult {
   if let Some(result) = extract_name(source) {
      Some(result)
   } else if let Some(result) = extract_number(source) {
      Some(result)
   } else if let Some(result) = extract_char_token(source) {
      Some(result)
   } else {
      None
   }
}


fn extract_name(source: &str) -> TokenResult {
   let mut end = 0;

   let mut chars = source.chars();

   match chars.next() {
      Some(ch) => match ch {
         'a'...'z' | 'A' ... 'Z' => end += 1,
         _ => return None,
      },
      None => return None,
   }

   loop {
      match chars.next() {
         Some(ch) => match ch {
            'a'...'z' | 'A' ... 'Z' | '0' ... '1' | '-' | '_' => end += 1,
            _ => break,
         },
         None => break,
      }
   }

   let token = Token::Name(
      String::from(&source[0..end])
   );

   Some((token, &source[end..]))
}


fn extract_char_token(source: &str) -> TokenResult {
   let mut chars = source.chars();

   match chars.next() {
      Some(ch) => {
         let token = match ch {
            '\n' => Token::NewLine,
            ',' => Token::Comma,
            '=' => Token::Assign,
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::ParenLeft,
            ')' => Token::ParenRight,
            '[' => Token::BracketLeft,
            ']' => Token::BracketRight,
            _ => return None,
         };

         Some((token, &source[1..]))
      },
      None => None,
   }
}


fn extract_number(source: &str) -> TokenResult {
   let mut end = 0;

   let positive = match source.chars().next() {
      None => {
         return None;
      },
      Some('-') => {
         end += 1;
         false
      },
      Some(_) => {
         true
      },
   };

   let mut integral = 0;

   let source = &source[end..];
   end = 0;

   for ch in source.chars() {
      if let Some(digit) = to_digit(ch) {
         integral = 10 * integral + digit;
         end += 1;
      } else {
         break;
      }
   }

   if end == 0 {
      return None;
   }

   let source = &source[end..];
   end = 0;

   match source.chars().next() {
      Some('.') => {
         end += 1;
      },
      _ => {
         if !positive {
            integral = -integral
         }

         return Some((Token::Integer(integral), source));
      }
   }

   let source = &source[end..];

   end = 0;

   let mut fractional = 0;
   let mut divisor = 1;

   for ch in source.chars() {
      if let Some(digit) = to_digit(ch) {
         fractional = 10 * fractional + digit;
         divisor = 10 * divisor;

         end += 1;
      } else {
         break;
      }
   }

   let mut value = integral as f64 + fractional as f64 / divisor as f64;

   if !positive {
      value = -value
   }

   let rest = &source[end..];

   Some((Token::Float(value), rest))
}


fn to_digit(ch: char) -> Option<i64> {
   match ch {
      '0' ... '9' => Some((ch as u32 - '0' as u32) as i64),
      _ =>  None
   }
}



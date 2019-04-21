use crate::rush::Rush::{Bin, Empty};

pub enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let tokens = tokenize(line);
    let mut iter = tokens.iter();
    if let Some(c) = iter.next() {
      let args = iter.map(|s| s.to_string()).collect::<Vec<_>>();
      Bin(c.to_string(), args)
    } else {
      Empty
    }
  }
}

fn tokenize(input: String) -> Vec<String> {
  let mut quote = false;
  let mut escape = false;
  let mut token = String::new();
  let mut tokens = Vec::new();

  for c in input.chars() {
    match c {
      '"' if !escape => {
        quote = !quote;
      }
      ' ' if !escape && !quote => {
        tokens.push(token);
        token = String::new();
      }
      '\\' => escape = true,
      _ => {
        token.push(c);
        escape = false;
      }
    }
  }
  tokens.push(token);
  tokens
}
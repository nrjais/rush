use crate::token::Token::{Empty, Value};

#[derive(Clone)]
pub enum Token {
  Value(String),
  Empty,
}

impl Token {
  pub fn from(s: &str) -> Self {
    if s.is_empty() {
      Empty
    } else {
      Value(s.to_owned())
    }
  }

  pub fn value(&self) -> Option<String> {
    match self {
      Token::Value(s) => Some(s.to_owned()),
      _ => None,
    }
  }

  pub fn is_var(&self) -> bool {
    match self {
      Token::Value(s) => s.starts_with("$"),
      _ => false,
    }
  }

  pub fn expand(&self) -> Token {
    match self {
      Token::Value(s) if s.contains("$") => Token::Value(replace_with_value(s)),
      _ => self.clone(),
    }
  }
}

fn replace_with_value(s: &String) -> String {
  let x = s.matches("\\$[A-Z_]+");
  println!("{:?}", x);
  String::new()
}
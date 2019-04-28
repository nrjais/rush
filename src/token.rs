use crate::token::Token::{Empty, Value};
use std::env;
use std::env::VarError;

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
  parse_vars(s)
      .iter_mut()
      .map(|t| expand(t, |s| env::var(s)))
      .collect()
}

fn expand(v: &mut String, resolver: fn(&str) -> Result<String, VarError>) -> String {
  match v.chars().next() {
    None => String::new(),
    Some('$') => {
      resolver(v.trim_start_matches('$')).unwrap_or_default()
    }
    Some('\\') => v.trim_start_matches('\\').to_owned(),
    _ => v.to_owned()
  }
}

fn parse_vars(s: &String) -> Vec<String> {
  let mut matching = false;
  let mut m = String::new();
  let mut matches = Vec::new();
  let mut escaped = false;

  for c in s.chars() {
    match c {
      '$' if !escaped => {
        matching = true;
        matches.push(m);
        m = String::from("$");
      }
      '$' => {
        matches.push(m);
        m = String::from("\\$")
      }
      '\\' => {
        escaped = true;
      }
      'a'..='z' | 'A'..='Z' | '0'..='9' | '_' if matching => m.push(c),
      _ if matching => {
        matches.push(m);
        m = String::new();
        m.push(c);
      }
      _ => {
        m.push(c);
      }
    }
  }

  matches.push(m);
  matches
}

#[cfg(test)]
pub mod tests {
  use crate::token::{parse_vars, expand};

  #[test]
  fn should_parse_empty_string() {
    let parsed = parse_vars(&String::from(""));
    assert_eq!(vec![String::new()], parsed);
  }

  #[test]
  fn should_parse_string_word() {
    let parsed = parse_vars(&String::from("hello"));
    assert_eq!(vec![String::from("hello")], parsed);
  }

  #[test]
  fn should_parse_string_with_single_var() {
    let parsed = parse_vars(&String::from("$hello"));
    assert_eq!(vec![String::from(""), String::from("$hello")], parsed);
  }

  #[test]
  fn should_parse_string_with_multiple_var() {
    let parsed = parse_vars(&String::from("$hello$world"));
    assert_eq!(vec![String::from(""), String::from("$hello"), String::from("$world")], parsed);
  }

  #[test]
  fn should_parse_string_with_multiple_separated_var() {
    let parsed = parse_vars(&String::from("$hello-$world"));
    assert_eq!(vec![String::from(""), String::from("$hello"), String::from("-"), String::from("$world")], parsed);
  }

  #[test]
  fn should_parse_with_escaped_var() {
    let parsed = parse_vars(&String::from("$hello-\\$world"));
    assert_eq!(vec![String::from(""), String::from("$hello"), String::from("-"), String::from("\\$world")], parsed);
  }

  #[test]
  fn should_expand_var(){
    let mut var = String::from("$H");
    let actual = expand(&mut var, |_| Ok("Hello".to_owned()));
    assert_eq!(String::from("Hello"), actual)
  }

  #[test]
  fn should_not_expand_var(){
    let mut var = String::from("H");
    let actual = expand(&mut var, |_| Ok("Hello".to_owned()));
    assert_eq!(String::from("H"), actual)
  }

  #[test]
  fn should_not_expand_var_when_escaped(){
    let mut var = String::from("\\$H");
    let actual = expand(&mut var, |_| Ok("Hello".to_owned()));
    assert_eq!(String::from("$H"), actual)
  }
}

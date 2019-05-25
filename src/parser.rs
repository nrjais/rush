use std::iter::Peekable;
use std::str::Chars;
use crate::token::Token;
use crate::token::Token::{Values, Value, Pipe};

trait VecExt<T> {
  fn push_value(&mut self, ele: T);
}

impl VecExt<Token> for Vec<Token> {
  fn push_value(&mut self, token: Token) {
    if token.value().is_some() {
      self.push(token)
    }
  }
}

pub fn parse(input: String) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut iter = input.chars().peekable();

  while let Some(c) = iter.peek() {
    let token = match c {
      '"' => consume_multiple_strings(&mut iter),
      _ => consume_word(&mut iter),
    };
    tokens.push_value(token);
  }

  tokens
}

fn consume_word(iter: &mut Peekable<Chars>) -> Token {
  let mut token_string = String::new();
  while let Some(c) = iter.next() {
    let next = iter.peek().map(|c| c.to_owned());
    match c {
      '\\' if next == Some(' ') => {
        token_string.push(iter.next().unwrap_or_default())
      }
      ' ' => break,
      _ => token_string.push(c)
    }
  }

  token_from(token_string)
}

fn token_from(s: String) -> Token {
  match s.trim() {
    "|" => Pipe,
    _ => Value(s)
  }
}

fn consume_multiple_strings(iter: &mut Peekable<Chars>) -> Token {
  let mut values = Vec::new();
  while let Some(c) = iter.peek() {
    match c {
      '"' => values.push(consume_string(iter)),
      _ => break
    }
  }

  Values(values)
}

fn consume_string(iter: &mut Peekable<Chars>) -> String {
  let mut token = String::new();
  let _ = iter.next(); // Ignore first already seen quote
  while let Some(c) = iter.next() {
    let next = iter.peek().map(|c| c.to_owned());
    match c {
      '\\' if next == Some('"') => {
        token.push(iter.next().unwrap_or_default())
      }
      '"' => break,
      _ => token.push(c)
    }
  }

  token
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//*****Tests**************************************************************************************//
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
  use crate::parser::parse;
  use crate::token::Token;
  use crate::token::Token::*;

  #[test]
  fn should_parse_empty_string() {
    let actual = parse("".to_owned());
    let expected: Vec<Token> = vec![];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_multiple_words() {
    let actual = parse("hello world".to_owned());
    let expected: Vec<Token> = vec![Value(String::from("hello")), Value(String::from("world"))];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_word_with_string() {
    let actual = parse("echo \"hello world\"".to_owned());
    let expected: Vec<Token> = vec![Value(String::from("echo")), Values(vec![String::from("hello world")])];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_word_with_escaped_space_string() {
    let actual = parse("echo hello\\ world".to_owned());
    let expected: Vec<Token> = vec![Value(String::from("echo")), Value(String::from("hello world"))];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_word_joined_strings() {
    let actual = parse("echo \"hello\"\"world\"".to_owned());
    let expected: Vec<Token> = vec![Value(String::from("echo")), Values(vec![String::from("hello"), String::from("world")])];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_with_pipe() {
    let actual = parse("echo hello | cat".to_owned());
    let expected: Vec<Token> = vec![Value(String::from("echo")), Value(String::from("hello")), Pipe, Value(String::from("cat"))];
    assert_eq!(expected, actual)
  }
}

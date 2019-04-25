use crate::rush::Rush::*;
use crate::token::Token;
use crate::token::Token::Value;

pub enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let tokens = expand_vars(tokenize(line));
    let mut iter = tokens.iter().filter_map(|t| t.value());
    if let Some(c) = iter.next() {
      Bin(c, iter.collect())
    } else {
      Empty
    }
  }
}

fn tokenize(input: String) -> Vec<Token> {
  let mut quote = false;
  let mut escape = false;
  let mut token = String::new();
  let mut tokens = Vec::new();

  for c in input.chars() {
    match c {
      '"' if !escape => quote = !quote,
      '\\' => escape = true,
      ' ' if !escape && !quote => {
        tokens.push(Value(token));
        token = String::new();
      }
      _ => {
        token.push(c);
        escape = false;
      }
    }
  }
  tokens.push(Value(token));
  tokens
}

fn expand_vars(tokens: Vec<Token>) -> Vec<Token> {
  let mut expanded_tokens = Vec::new();
  for token in tokens.iter() {
    expanded_tokens.push(token.expand());
  }
  expanded_tokens
}
use std::env;

use crate::rush::Rush::{Bin, Empty};

pub enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let tokens = expand_vars(tokenize(line));
    let mut iter = tokens.iter();
    if let Some(c) = iter.next() {
      let args = iter.map(|s| s.to_string()).collect();
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

fn expand_vars(tokens: Vec<String>) -> Vec<String> {
  let mut expanded_tokens = Vec::new();
  for token in tokens.iter() {
    expanded_tokens.push(expand(token).to_owned());
  }
  expanded_tokens
}

fn expand(token: &str) -> String {
  if token.starts_with('$') {
    env::var(token.trim_start_matches('$')).unwrap_or_default()
  } else {
    token.to_owned()
  }
}
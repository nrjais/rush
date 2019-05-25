use crate::parser::parse;
use crate::rush::Rush::*;
use crate::token::Token::Pipe;

#[derive(Debug)]
pub enum Rush {
  Bin(String, Vec<String>),
  Piped(Vec<Rush>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let parsed = parse(line);
    let tokens: Vec<String> = parsed
        .iter()
        .filter_map(|t| t.expand().value())
        .collect();

    if parsed.contains(&Pipe) {
      partition(tokens)
    } else if tokens.len() > 0 {
      bin(tokens)
    } else {
      Empty
    }
  }
}

fn bin(tokens: Vec<String>) -> Rush {
  let mut iter = tokens.iter();
  Bin(iter.next().unwrap().to_owned(), iter.map(|s| s.to_owned()).collect())
}

fn partition(tokens: Vec<String>) -> Rush {
  let mut s = Vec::new();
  let mut bins = Vec::new();
  for token in tokens {
    if token == "|".to_owned() {
      bins.push(bin(s));
      s = Vec::new();
    } else {
      s.push(token)
    }
  }
  bins.push(bin(s));
  Piped(bins)
}

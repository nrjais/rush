use crate::parser::parse;
use crate::rush::Rush::*;

pub enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let tokens = parse(line);
    let mut iter = tokens
        .iter()
        .filter_map(|t| t.expand().value());
    if let Some(c) = iter.next() {
      Bin(c, iter.collect())
    } else {
      Empty
    }
  }
}

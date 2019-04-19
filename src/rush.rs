use crate::rush::Rush::{Bin, Empty};

pub enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

impl Rush {
  pub fn from(line: String) -> Rush {
    let mut iter = line.split_whitespace();
    if let Some(c) = iter.next() {
      let args = iter.map(|s| s.to_string()).collect::<Vec<_>>();
      Bin(c.to_string(), args)
    } else {
      Empty
    }
  }
}
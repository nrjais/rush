use crate::rush::Rush::*;

#[derive(Debug)]
pub enum Rush {
  Bin(String, Vec<String>),
  Piped(Vec<Rush>),
  Empty,
}

impl Rush {
  pub fn from(_line: String) -> Rush {
    Empty
  }
}

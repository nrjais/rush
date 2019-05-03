use crate::token::Token;

trait VecExt<T> {
  fn push_value(&mut self, ele: T);
}

pub fn parse(input: String) -> Vec<Token> {
  let mut quote = false;
  let mut escape = false;
  let mut token = String::new();
  let mut tokens = Vec::new();

  let mut iter = input.chars().peekable();

  while let Some(c) = iter.next() {
    match c {
      '"' if !escape => quote = !quote,
      '\\' => escape = true,
      ' ' if !escape && !quote => {
        tokens.push_value(Token::from(token));
        token = String::new();
      }
      _ => {
        token.push(c);
        escape = false;
      }
    }
  }

  tokens.push_value(Token::from(token));
  tokens
}

impl VecExt<Token> for Vec<Token> {
  fn push_value(&mut self, token: Token) {
    if token.value().is_some() {
      self.push(token)
    }
  }
}

#[cfg(test)]
pub mod tests {
  use crate::parser::parse;
  use crate::token::Token;

  #[test]
  fn should_parse() {
    let actual = parse("".to_owned());
    let expected: Vec<Token> = vec![];
    assert_eq!(expected, actual)
  }
}
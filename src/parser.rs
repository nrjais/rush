use nom::{
  branch::alt,
  bytes::complete::{escaped, take_while, take_while1},
  character::complete::{alphanumeric1 as alphanumeric, char, one_of},
  combinator::{cut, map},
  error::{context, ErrorKind, ParseError},
  IResult,
  sequence::{preceded, terminated},
};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Var(&'a str),
  Literal(&'a str),
  Substitution(&'a str, Vec<Token<'a>>),
  Pipe,
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";
  take_while(move |c| chars.contains(c))(i)
}

fn is_valid_var_char(c: char) -> bool {
  match c {
    '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => true,
    _ => false,
  }
}

fn var<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Token<'a>, E> {
  context(
    "var",
    map(
      preceded(char('$'), take_while1(is_valid_var_char)),
      Token::Var,
    ),
  )(i)
}

fn literal<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Token<'a>, E> {
  map(string, Token::Literal)(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  escaped(alphanumeric, '\\', one_of("\"n\\"))(i)
}

fn string<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  context(
    "string",
    preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
  )(i)
}

fn pipe<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Token<'a>, E> {
  context("pipe", map(char('|'), |_| Token::Pipe))(i)
}

fn segment<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Token<'a>, E> {
  context("segment", preceded(sp, alt((literal, pipe, var))))(i)
}

fn root<'a, E: ParseError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<Token<'a>>, E> {
  map(segment, |tuple_vec| vec![tuple_vec])(i)
}

pub fn parse(i: &str) -> Vec<Token> {
  let result = root::<(&str, ErrorKind)>(i);
  println!("{:#?}", result);
  vec![]
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//*****Tests**************************************************************************************//
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
  use crate::parser::{parse, Token};
  use crate::parser::Token::Literal;

  #[test]
  fn should_parse_empty_string() {
    let actual = parse("");
    let expected: Vec<Token> = vec![];
    assert_eq!(expected, actual)
  }

  #[test]
  fn should_parse_multiple_words() {
    let actual = parse(r#""hello""#);
    let expected: Vec<Token> = vec![Literal("hello world")];
    assert_eq!(actual, expected)
  }
  //
  //  #[test]
  //  fn should_parse_word_with_string() {
  //    let actual = parse("echo \"hello world\"".to_owned());
  //    let expected: Vec<Token> = vec![Value(String::from("echo")), Values(vec![String::from("hello world")])];
  //    assert_eq!(expected, actual)
  //  }
  //
  //  #[test]
  //  fn should_parse_word_with_escaped_space_string() {
  //    let actual = parse("echo hello\\ world".to_owned());
  //    let expected: Vec<Token> = vec![Value(String::from("echo")), Value(String::from("hello world"))];
  //    assert_eq!(expected, actual)
  //  }
  //
  //  #[test]
  //  fn should_parse_word_joined_strings() {
  //    let actual = parse("echo \"hello\"\"world\"".to_owned());
  //    let expected: Vec<Token> = vec![Value(String::from("echo")), Values(vec![String::from("hello"), String::from("world")])];
  //    assert_eq!(expected, actual)
  //  }
  //
  //  #[test]
  //  fn should_parse_with_pipe() {
  //    let actual = parse("echo hello | cat".to_owned());
  //    let expected: Vec<Token> = vec![Value(String::from("echo")), Value(String::from("hello")), Pipe, Value(String::from("cat"))];
  //    assert_eq!(expected, actual)
  //  }
}

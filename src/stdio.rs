use std::io;
use std::io::Write;

pub fn read_line() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

pub fn print(string: String) {
  io::stdout().write(string.as_bytes()).unwrap();
  io::stdout().flush().unwrap();
}

pub fn println(string: String) {
  print(string + "\n");
}
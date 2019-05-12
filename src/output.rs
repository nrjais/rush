use std::io;
use std::io::Write;

fn write_to<T>(stream: &mut T, string: String) where T: Write {
  stream.write(string.as_bytes()).unwrap();
  stream.flush().unwrap();
}

pub fn print(string: String) {
  write_to(&mut io::stdout(), string);
}

pub fn print_err(string: String) {
  write_to(&mut io::stderr(), string);
}

pub fn println(string: String) {
  print(string + "\n");
}

pub fn println_err(string: String) {
  print_err(string + "\n");
}
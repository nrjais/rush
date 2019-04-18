use std::io;
use std::io::Write;
use std::str;
use std::process::Command;
use Rush::*;

enum Rush {
  Bin(String, Vec<String>),
  Empty,
}

fn main() {
  let greeting = "Welcome to RUSH.......";
  let prompt = String::from("#>");
  println!("{}", greeting);

  loop {
    print(prompt.clone());

    let command = parse_command(read_line());
    let result = match command {
      Rush::Bin(c, a) => {
        Command::new(c)
            .args(a)
            .env_clear()
            .output()
            .map(|o| str::from_utf8(o.stdout.as_slice()).unwrap().to_string())
      }
      Rush::Empty => {
        Result::Ok(String::new())
      }
    };

    match result {
      Ok(output) => print(output),
      Err(err) => print(err.to_string())
    }
  }
}

fn parse_command(line: String) -> Rush {
  let mut iter = line.split_whitespace();
  if let Some(c) = iter.next() {
    let args = iter.map(|s| s.to_string()).collect::<Vec<_>>();
    Bin(c.to_string(), args)
  } else {
    Empty
  }
}

fn read_line() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

fn print(string: String) {
  io::stdout().write(string.as_bytes()).unwrap();
  io::stdout().flush().unwrap();
}

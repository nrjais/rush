use std::io;
use std::io::Write;
use std::str;
use std::process::Command;
use rush::rush::Rush;
use Rush::*;
use rush::stdio::*;

fn main() {
  let greeting = "Welcome to RUSH.......";
  let prompt = String::from("≈>");

  println!("{}", greeting);

  let mut pwd = "/".to_string();

  loop {
    print(prompt.clone());

    let command = Rush::from(read_line());
    let result = match command {
      Rush::Bin(c, a) => Command::new(c)
          .env_clear()
          .args(a)
          .current_dir(pwd.as_str())
          .output()
          .map(|o| String::from_utf8(o.stdout).unwrap()),
      Rush::Empty => Result::Ok(String::new()),
    };

    match result {
      Ok(output) => print(output),
      Err(err) => println(err.to_string())
    }
  }
}


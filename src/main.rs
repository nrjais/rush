use std::process::Command;
use rush::rush::Rush;
use rush::stdio::*;
use std::env;

fn main() {
  let greeting = "Welcome to RUSH.......";
  println!("{}", greeting);

  let prompt = String::from("≈>");

  let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();

  loop {
    print(current_dir.clone() + prompt.as_str());

    let command = Rush::from(read_line());
    let result = match command {
      Rush::Bin(c, a) => Command::new(c)
          .args(a)
          .output()
          .map(|o| String::from_utf8(o.stdout).unwrap_or_default()),
      Rush::Empty => Result::Ok(String::new()),
    };

    match result {
      Ok(output) => print(output),
      Err(err) => println(err.to_string())
    }
  }
}


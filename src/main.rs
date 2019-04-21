use std::process::Command;
use rush::rush::Rush;
use rush::stdio::*;
use std::env;
use std::io::Error;
use std::collections::HashMap;
use std::path::Path;

fn main() {
  let greeting = "Welcome to RUSH.......";
  println!("{}", greeting);

  loop {
    print(build_prompt());
    match launch(Rush::from(read_line())) {
      Ok(output) => print(output),
      Err(err) => println(err.to_string())
    }
  }
}

fn build_prompt() -> String {
  let prompt = "≈>";
  env::current_dir().unwrap().to_string_lossy().to_string() + prompt
}

fn launch(command: Rush) -> Result<String, Error> {
  let mut builtins = HashMap::new();
  builtins.insert(String::from("cd"), builtin_cd);


  match command {
    Rush::Bin(c, a) => {
      if let Some(builtin) = builtins.get(&c) {
        builtin(a)
      } else {
        Command::new(c)
            .args(a)
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
      }
    }
    Rush::Empty => Result::Ok(String::new()),
  }
}

fn builtin_cd(args: Vec<String>) -> Result<String, Error> {
  let home = String::from("~");
  let path = args.get(0).unwrap_or(&home);
  env::set_current_dir(Path::new(path))
      .map(|_| String::new())
}


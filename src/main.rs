use std::process::Command;
use rush::rush::Rush;
use rush::stdio::*;
use std::env;
use std::io::Error;
use rush::builtins::builtins;

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
  match command {
    Rush::Bin(cmd, args) => {
      builtins()
          .get(&cmd)
          .map_or_else(|| execute(cmd, args.clone()),
                       |builtin| builtin(args.clone()))
    }
    Rush::Empty => Result::Ok(String::new()),
  }
}

fn execute(cmd: String, args: Vec<String>) -> Result<String, Error> {
  Command::new(cmd)
      .args(args)
      .output()
      .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
}



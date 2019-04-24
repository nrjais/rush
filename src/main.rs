use std::process::{Command, Output, ExitStatus};
use rush::rush::Rush;
use rush::stdio::*;
use std::env;
use std::io::Error;
use rush::builtins::builtins;
use std::os::unix::process::ExitStatusExt;

fn main() {
  let greeting = "Welcome to RUSH.......";
  println!("{}", greeting);

  loop {
    print(build_prompt());
    match launch(Rush::from(read_line())) {
      Ok(output) => {
        print(String::from_utf8(output.stdout).unwrap_or_default());
      }
      Err(err) => println_err(err.to_string())
    }
  }
}

fn build_prompt() -> String {
  let prompt = "≈>";
  env::current_dir().unwrap().to_string_lossy().to_string() + prompt
}

fn launch(command: Rush) -> Result<Output, Error> {
  match command {
    Rush::Bin(cmd, args) => {
      builtins()
          .get(&cmd)
          .map_or_else(|| execute(cmd, args.clone()),
                       |builtin| builtin(args.clone()))
    }
    Rush::Empty => Ok(Output {
      stderr: Vec::new(),
      stdout: Vec::new(),
      status: ExitStatus::from_raw(0),
    }),
  }
}

fn execute(cmd: String, args: Vec<String>) -> Result<Output, Error> {
  Command::new(cmd)
      .args(args)
      .spawn()
      .map(|c| c.wait_with_output())?
}



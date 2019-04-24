use std::env;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

use rush::builtins::builtins;
use rush::rush::Rush;
use rush::stdio::*;

fn main() {
  let greeting = "Welcome to RUSH.......";
  println!("{}", greeting);

  loop {
    print(build_prompt());
    match launch(Rush::from(read_line())) {
      Ok(status) => {
        if let Some(code) = status.code() {
          env::set_var("STATUS", code.to_string())
        }
      }
      Err(_) => {
        env::set_var("STATUS", 127.to_string());
        println_err("Command not found".to_owned())
      }
    }
  }
}

fn build_prompt() -> String {
  let prompt = "≈>";
  env::current_dir().unwrap().to_string_lossy().to_string() + prompt
}

fn launch(command: Rush) -> Result<ExitStatus, Error> {
  match command {
    Rush::Bin(cmd, args) => {
      builtins()
          .get(&cmd)
          .map_or_else(|| execute(cmd, args.clone()),
                       |builtin| builtin(args.clone()))
    }
    Rush::Empty => Ok(ExitStatus::from_raw(0)),
  }
}

fn execute(cmd: String, args: Vec<String>) -> Result<ExitStatus, Error> {
  Command::new(cmd)
      .args(args)
      .spawn()
      .map(|mut c| c.wait())?
}



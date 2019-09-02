extern crate rustyline;

use std::env;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command, ExitStatus, Stdio};

use rustyline::{Editor, Config, CompletionType, EditMode, OutputStreamType, Context};
use rustyline::error::ReadlineError;

use rush::builtins::builtins;
use rush::output::println_err;
use rush::rush::Rush;
use rustyline::completion::{FilenameCompleter, Completer, Pair};
use rustyline::highlight::{MatchingBracketHighlighter, Highlighter};
use rustyline::hint::{HistoryHinter, Hinter};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use rustyline_derive::Helper;

#[derive(Helper)]
struct MyHelper {
  completer: FilenameCompleter,
  highlighter: MatchingBracketHighlighter,
  hinter: HistoryHinter,
  colored_prompt: String,
}

impl Completer for MyHelper {
  type Candidate = Pair;

  fn complete(
    &self,
    line: &str,
    pos: usize,
    ctx: &Context<'_>,
  ) -> Result<(usize, Vec<Pair>), ReadlineError> {
    self.completer.complete(line, pos, ctx)
  }
}

impl Hinter for MyHelper {
  fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
    self.hinter.hint(line, pos, ctx)
  }
}

impl Highlighter for MyHelper {
  fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
    self.highlighter.highlight(line, pos)
  }

  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
    &'s self,
    prompt: &'p str,
    default: bool,
  ) -> Cow<'b, str> {
    if default {
      Borrowed(&self.colored_prompt)
    } else {
      Borrowed(prompt)
    }
  }

  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
  }

  fn highlight_char(&self, line: &str, pos: usize) -> bool {
    self.highlighter.highlight_char(line, pos)
  }
}


fn main() {
  let greeting = "Welcome to RUSH.......";
  println!("{}", greeting);
  let config = Config::builder()
      .history_ignore_space(true)
      .auto_add_history(true)
      .completion_type(CompletionType::List)
      .edit_mode(EditMode::Emacs)
      .output_stream(OutputStreamType::Stdout)
      .build();

  let mut rl = Editor::with_config(config);
  let h = MyHelper {
    completer: FilenameCompleter::new(),
    highlighter: MatchingBracketHighlighter::new(),
    hinter: HistoryHinter {},
    colored_prompt: "\x1b[37m".to_owned(),
  };
  rl.set_helper(Some(h));

  loop {
    let readline = rl.readline(build_prompt().as_str());
    match readline {
      Ok(line) => {
        match launch(Rush::from(line)) {
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
      Err(ReadlineError::Interrupted) => continue,
      Err(ReadlineError::Eof) => break,
      Err(err) => println!("Error: {:?}", err),
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
    Rush::Piped(mut commands) => {
      let last = commands.pop();
      let x = commands
          .iter()
          .fold(None, |r: Option<Child>, c| {
            let stdin = r.map(|c| Stdio::from(c.stdout.unwrap()))
                .unwrap_or(Stdio::inherit());
            spawn_c(c, stdin, Stdio::piped())
          })
          .unwrap();

      spawn_c(&last.unwrap(), Stdio::from(x.stdout.unwrap()), Stdio::inherit())
          .unwrap()
          .wait()
    }
  }
}

fn spawn_c(r: &Rush, stdin: Stdio, stdout: Stdio) -> Option<Child> {
  match r {
    Rush::Bin(cmd, args) => spawn(cmd, args, stdin, stdout).ok(),
    _ => None
  }
}

fn execute(cmd: String, args: Vec<String>) -> Result<ExitStatus, Error> {
  spawn(&cmd, &args, Stdio::inherit(), Stdio::inherit())
      .map(|mut c| c.wait())?
}

fn spawn(cmd: &String, args: &Vec<String>, stdin: Stdio, stdout: Stdio) -> Result<Child, Error> {
  Command::new(cmd)
      .args(args)
      .stdin(stdin)
      .stdout(stdout)
      .spawn()
}



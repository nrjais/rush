extern crate dirs;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::path::PathBuf;

type Builtin = fn(Vec<String>) -> Result<String, Error>;

pub fn builtins() -> HashMap<String, Builtin, RandomState> {
  let mut builtins: HashMap<String, Builtin, RandomState> = HashMap::new();

  builtins.insert(String::from("cd"), builtin_cd);

  builtins
}

fn builtin_cd(args: Vec<String>) -> Result<String, Error> {
  let path = args.get(0)
      .map(PathBuf::from)
      .unwrap_or_else(|| dirs::home_dir().unwrap());

  env::set_current_dir(path).map(|_| String::new())
}
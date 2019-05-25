extern crate termion;

use std::io;
use std::io::{Stdout, Write};

use termion::input::TermRead;
use termion::raw::IntoRawMode;

use self::termion::event::Key;
use self::termion::raw::RawTerminal;
use std::collections::vec_deque::VecDeque;

struct Input {
  cursor: u16,
  left: String,
  right: VecDeque<u8>,
  s: RawTerminal<Stdout>,
}

impl Input {
  fn get(mut self) -> String {
    writeln!(self.s, "\r").unwrap();
    let right = self.right_string();
    self.left + right.as_str()
  }
  fn new(s: RawTerminal<Stdout>) -> Input {
    Input {
      cursor: 0,
      left: String::new(),
      right: VecDeque::new(),
      s,
    }
  }

  fn right_string(&self) -> String {
    String::from_utf8(Vec::from(self.right.clone())).unwrap()
  }

  fn push(&mut self, c: char) {
    self.left.push(c);
    write!(self.s, "{}", c).unwrap();
    self.show_right();
    if self.cursor > 0 {
      write!(self.s, "{}", termion::cursor::Left(self.cursor)).unwrap();
    }
    self.flush();
  }

  fn show_right(&mut self) {
    write!(self.s, "{}{}", termion::clear::AfterCursor, self.right_string()).unwrap();
  }

  fn flush(&mut self) {
    self.s.flush().unwrap();
  }

  fn left(&mut self) {
    if let Some(c) = self.left.pop() {
      self.cursor += 1;
      self.right.push_front(c as u8);
      write!(self.s, "{}", termion::cursor::Left(1)).unwrap();
      self.flush();
    }
  }

  fn right(&mut self) {
    if let Some(c) = self.right.pop_front() {
      self.cursor -= 1;
      self.left.push(c as char);
      write!(self.s, "{}", termion::cursor::Right(1)).unwrap();
      self.flush();
    }
  }

  fn backspace(&mut self) {
    if self.left.pop().is_some() {
      write!(self.s, "{}", termion::cursor::Left(1)).unwrap();
      self.show_right();
      if self.cursor > 0 {
        write!(self.s, "{}", termion::cursor::Left(self.cursor)).unwrap();
      }
      self.flush();
    }
  }
}

pub fn read_line() -> String {
//  String::from("echo hello | grep h")
   let mut input = Input::new(io::stdout().into_raw_mode().unwrap());

   for key in io::stdin().keys() {
     match key.unwrap() {
       Key::Backspace => { input.backspace() }
       Key::Left => { input.left() }
       Key::Right => { input.right() }
       Key::Char(c) if c == '\n' => { break; }
       Key::Char(c) => {
         input.push(c);
       }
       _ => {}
     }
   }
   input.get()
}

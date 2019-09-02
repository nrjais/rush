extern crate termion;

use std::collections::vec_deque::VecDeque;
use std::io;
use std::io::{Stdout, Write};

use termion::input::TermRead;
use termion::raw::IntoRawMode;

use self::termion::cursor::DetectCursorPos;
use self::termion::event::Key;
use self::termion::raw::RawTerminal;

const DEFAULT_TERM_SIZE: (u16, u16) = (80, 80);

struct InputContext {
  s: RawTerminal<Stdout>,
  left: String,
  right: VecDeque<u8>,
  prompt: String,
  cursor: u16,
  cursor_pos: (u16, u16),
  cursor_edit: (u16, u16),
  term_size: (u16, u16),
  rows: u16,
}

impl InputContext {
  fn get(mut self) -> String {
    writeln!(self.s, "\r").unwrap();
    let right = self.right_string();
    self.left + right.as_str()
  }

  fn new(prompt: String, mut s: RawTerminal<Stdout>) -> InputContext {
    let cursor_pos = s.cursor_pos().unwrap();
    let mut cursor_edit = cursor_pos;
    let prompt_len = prompt.len() as u16;
    cursor_edit.0 = cursor_edit.0 + prompt_len - 2u16;

    InputContext {
      cursor: prompt_len,
      prompt,
      left: String::new(),
      right: VecDeque::new(),
      s,
      cursor_pos,
      cursor_edit,
      term_size: termion::terminal_size().unwrap_or(DEFAULT_TERM_SIZE),
      rows: 1,
    }
  }

  fn right_string(&self) -> String {
    String::from_utf8(Vec::from(self.right.clone())).unwrap()
  }

  fn push(&mut self, c: char) {
    self.left.push(c);
    self.incr_cursor_pos();
  }

  fn left(&mut self) {
    if let Some(c) = self.left.pop() {
      self.right.push_front(c as u8);
      self.decr_cursor_pos();
    }
  }

  fn right(&mut self) {
    if let Some(c) = self.right.pop_front() {
      self.left.push(c as char);
      self.incr_cursor_pos();
    }
  }

  fn backspace(&mut self) {
    if self.left.pop().is_some() {
      self.decr_cursor_pos();
    }
  }

  fn incr_cursor_pos(&mut self) {
    self.cursor += 1;
    self.cursor_edit.0 += 1;

    if self.cursor_edit.0 == (self.term_size.0 + 1) {
      self.cursor_edit.1 += 1;
      self.cursor_edit.0 = 1;
      self.rows += 1;
    }

    if self.cursor_edit.1 > self.term_size.1 {
      self.cursor_edit.1 -= 1;
      self.cursor_pos.1 -= 1;
    }

    self.refresh_line();
  }

  fn decr_cursor_pos(&mut self) {
    if self.cursor_edit.0 == 1 && self.rows > 1 {
      self.rows -= 1;
      self.cursor_edit.1 -= 1;
      self.cursor_edit.0 = self.term_size.0 + 1;
    }

    if self.cursor > self.prompt.len() as u16 {
      self.cursor_edit.0 -= 1;
      self.cursor -= 1;
    }

    self.refresh_line();
  }

  fn flush(&mut self) {
    self.s.flush().unwrap();
  }

  fn refresh_line(&mut self) {
    write!(self.s, "{}{}{}{}{}{}",
           termion::cursor::Goto(self.cursor_pos.0, self.cursor_pos.1),
           termion::clear::AfterCursor,
           self.prompt,
           self.left,
           self.right_string(),
           termion::cursor::Goto(self.cursor_edit.0, self.cursor_edit.1))
        .unwrap();
    self.flush();
  }
}

pub fn read_line(prompt: String) -> String {
  let mut input = InputContext::new(prompt, io::stdout().into_raw_mode().unwrap());
  input.refresh_line();

  for key in io::stdin().keys() {
    match key.unwrap() {
      Key::Backspace => input.backspace(),
      Key::Left => input.left(),
      Key::Right => input.right(),
      Key::Char(c) if c == '\n' => break,
      Key::Char(c) => input.push(c),
      _ => {}
    }
  }
  input.get()
}

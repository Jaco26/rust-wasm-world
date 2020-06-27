#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Button {
  UpArrow,
  RightArrow,
  DownArrow,
  LeftArrow,
  Spacebar,
  Return,
  Shift,
  Other,
}

impl Button {
  pub fn from_key_code(key_code: u32) -> Self {
    match key_code {
      38 => Self::UpArrow,
      39 => Self::RightArrow,
      40 => Self::DownArrow,
      37 => Self::LeftArrow,
      32 => Self::Spacebar,
      16 => Self::Shift,
      13 => Self::Return,
      _ => Self::Other,
    }
  }
}


pub struct UserInput {
  pressed_keys: HashMap<Button, usize>,
}

impl UserInput {
  fn priority(&self, btn: &Button) -> Option<usize> {
    match self.pressed_keys.get(btn) {
      Some(priority) => Some(*priority),
      None => None
    }
  }
  pub fn new(pressed_keys: Vec<u32>) -> UserInput {
    let accum: HashMap<Button, usize> = HashMap::new();
    UserInput {
      pressed_keys: pressed_keys
        .iter()
        .enumerate()
        .fold(accum, |mut acc, (priority, key_code)| {
          acc.insert(Button::from_key_code(*key_code), priority);
          acc
        }),
    }
  }
  pub fn has(&self, btn: &Button) -> bool {
    self.pressed_keys.contains_key(btn)
  }
  pub fn has_any(&self, buttons: Vec<&Button>) -> bool {
    buttons.iter().any(|btn| self.pressed_keys.contains_key(btn))
  }
  pub fn has_all(&self, buttons: Vec<&Button>) -> bool {
    buttons.iter().all(|btn| self.pressed_keys.contains_key(btn))
  }
  pub fn has_order(&self, b1: &Button, b2: &Button) -> Option<bool> {
    match (self.priority(b1), self.priority(b2)) {
      (Some(p1), Some(p2)) => Some(p1 < p2),
      (None, Some(_)) => Some(true),
      (Some(_), None) => Some(false),
      (None, None) => None,
    }
  }
}

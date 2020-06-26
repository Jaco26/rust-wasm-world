
#[derive(Debug, PartialEq)]
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
  pub pressed_keys: Vec<Button>,
}


impl UserInput {
  pub fn new(pressed_keys: Vec<u32>) -> UserInput {
    UserInput { 
      pressed_keys: pressed_keys
        .iter()
        .map(|x| Button::from_key_code(*x))
        .collect(),
    }
  }

  fn index_of(&self, key: &Button) -> Option<usize> {
    self.pressed_keys.iter().position(|x| x == key)
  }

  pub fn has(&self, btn: &Button) -> bool {
    self.pressed_keys.contains(&btn)
  }

  /// returns `true` if `first` has a smaller index than `second` in `self.pressed_keys`
  pub fn has_order(&self, first: &Button, second: &Button) -> bool {
    if let Some(first_idx) = self.index_of(&first) {
      if let Some(second_idx) = self.index_of(&second) {
        return first_idx < second_idx;
      }
    }
    false
  }

  pub fn has_all(&self, keys: Vec<&Button>) -> bool {
    keys.iter().all(|key| self.pressed_keys.contains(key))
  }
}

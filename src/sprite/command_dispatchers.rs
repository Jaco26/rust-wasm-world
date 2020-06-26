use crate::prelude::{
  Command,
  CommandDispatcher
};
use crate::user_input::{
  UserInput,
  Button as btn
};
use crate::sprite::commands as cmd;

fn dispatch_directional_commands(
  user_input: &UserInput,
  b1: &btn,
  b2: &btn,
  c1: Box<dyn Command>,
  c2: Box<dyn Command>,
  cancel: Box<dyn Command>
) -> Vec<Box<dyn Command>> {
  let mut rv: Vec<Box<dyn Command>> = Vec::new();
  if user_input.has_all(vec![b1, b2]) {
    match user_input.has_order(b1, b2) {
      true => rv.push(c2),
      false => rv.push(c1),
    };
  } else if user_input.has(b1) {
    rv.push(c1);
  } else if user_input.has(b2) {
    rv.push(c2);
  } else {
    rv.push(cancel);
  }
  rv
}

pub struct MovementX;
impl CommandDispatcher for MovementX {
  fn dispatch(&self, user_input: &UserInput) -> Vec<Box<dyn Command>> {
    dispatch_directional_commands(
      user_input,
      &btn::RightArrow,
      &btn::LeftArrow,
      Box::new(cmd::MoveRight),
      Box::new(cmd::MoveLeft),
      Box::new(cmd::CancelDx)
    )
  }
}

pub struct MovementY;
impl CommandDispatcher for MovementY {
  fn dispatch(&self, user_input: &UserInput) -> Vec<Box<dyn Command>> {
    dispatch_directional_commands(
      user_input,
      &btn::UpArrow,
      &btn::DownArrow,
      Box::new(cmd::MoveUp),
      Box::new(cmd::MoveDown),
      Box::new(cmd::CancelDy)
    )
  }
}
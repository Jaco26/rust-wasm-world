use crate::prelude::{
  Command,
  CommandDispatcher
};
use crate::user_input::{
  UserInput,
  Button as btn
};
use crate::sprite::commands as cmd;

fn dispatch_xy(
  user_input: &UserInput,
  b1: &btn,
  b2: &btn,
  c1: Box<dyn Command>,
  c2: Box<dyn Command>,
  cancel: Box<dyn Command>
) -> Box<dyn Command> {
  match user_input.has_order(b1, b2) {
    Some(bool_) if bool_ => c2,
    Some(_) => c1,
    None => cancel
  }
}

pub struct MovementXY;
impl CommandDispatcher for MovementXY {
  fn dispatch(&self, user_input: &UserInput) -> Vec<Box<dyn Command>> {
    let mut rv: Vec<Box<dyn Command>> = Vec::new();

    rv.push(dispatch_xy(
      user_input,
      &btn::UpArrow,
      &btn::DownArrow,
      Box::new(cmd::MoveUp),
      Box::new(cmd::MoveDown),
      Box::new(cmd::CancelDy),
    ));

    rv.push(dispatch_xy(
      user_input,
      &btn::RightArrow,
      &btn::LeftArrow,
      Box::new(cmd::MoveRight),
      Box::new(cmd::MoveLeft),
      Box::new(cmd::CancelDx),
    )); 

    rv
  }
}
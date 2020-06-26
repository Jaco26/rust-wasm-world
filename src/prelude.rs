use crate::sprite::Sprite;
use crate::user_input::UserInput;

pub trait GameActor {
  fn move_up(&mut self);
  fn move_down(&mut self);
  fn move_left(&mut self);
  fn move_right(&mut self);
  fn cancel_dx(&mut self);
  fn cancel_dy(&mut self);
}

pub trait Command {
  fn execute(&self, actor: &mut Sprite);
}

pub trait CommandDispatcher {
  fn dispatch(&self, user_input: &UserInput) -> Vec<Box<dyn Command>>;
}
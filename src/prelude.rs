use crate::sprite::Sprite;

pub trait GameActor {
  fn move_up(&mut self);
  fn move_down(&mut self);
  fn move_left(&mut self);
  fn move_right(&mut self);
  fn cancel_dx(&mut self);
  fn cancel_dy(&mut self);
}

// pub trait SpriteCommand {
//   fn execute(&self, actor: &mut Sprite);
// }

pub trait Command {
  fn execute(&self, actor: &mut Sprite);
}
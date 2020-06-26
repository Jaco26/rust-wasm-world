use crate::prelude::{Command, GameActor};
use crate::sprite::Sprite;

#[derive(Debug)]
pub struct MoveUp;
impl Command for MoveUp {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.move_up();
  }
}


#[derive(Debug)]
pub struct MoveRight;
impl Command for MoveRight {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.move_right();
  }
}

#[derive(Debug)]
pub struct MoveDown;
impl Command for MoveDown {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.move_down();
  }
}

#[derive(Debug)]
pub struct MoveLeft;
impl Command for MoveLeft {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.move_left();
  }
}

#[derive(Debug)]
pub struct CancelDx;
impl Command for CancelDx {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.cancel_dx();
  }
}

#[derive(Debug)]
pub struct CancelDy;
impl Command for CancelDy {
  fn execute(&self, sprite: &mut Sprite) {
    sprite.cancel_dy();
  }
}

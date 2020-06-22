use crate::sprite::{Sprite, GameActor};

pub trait Command {
  fn execute(&self, actor: &mut Sprite);
}


#[derive(Debug)]
struct MoveUp;
impl Command for MoveUp {
  fn execute(&self, actor: &mut Sprite) {
    actor.move_up();
  }
}

#[derive(Debug)]
struct MoveRight;
impl Command for MoveRight {
  fn execute(&self, actor: &mut Sprite) {
    actor.move_right();
  }
}

#[derive(Debug)]
struct MoveDown;
impl Command for MoveDown {
  fn execute(&self, actor: &mut Sprite) {
    actor.move_down();
  }
}

#[derive(Debug)]
struct MoveLeft;
impl Command for MoveLeft {
  fn execute(&self, actor: &mut Sprite) {
    actor.move_left();
  }
}

#[derive(Debug)]
struct CancelDx;
impl Command for CancelDx {
  fn execute(&self, actor: &mut Sprite) {
    actor.cancel_dx();
  }
}

#[derive(Debug)]
struct CancelDy;
impl Command for CancelDy {
  fn execute(&self, actor: &mut Sprite) {
    actor.cancel_dy();
  }
}

struct Button(Box<&'static dyn Command>);

pub struct InputHandler {
  buttons: std::collections::HashMap<u32, Button>
}


impl InputHandler {
  pub fn new() -> InputHandler {
    let mut buttons = std::collections::HashMap::new();
    buttons.insert(38, Button(Box::new(&MoveUp)));
    buttons.insert(39, Button(Box::new(&MoveRight)));
    buttons.insert(40, Button(Box::new(&MoveDown)));
    buttons.insert(37, Button(Box::new(&MoveLeft)));

    InputHandler { buttons }
  }
  
  pub fn handle_input(&mut self, pressed_keys: Vec<u32>) -> Vec<Box<&dyn Command>> {
    let mut command_stack = pressed_keys.iter().fold(Vec::new(), |mut acc, key_code| {
      if let Some(button) = self.buttons.get(key_code) {
        acc.push(button.0.clone());
      }
      acc
    });

    if !pressed_keys.contains(&38) && !pressed_keys.contains(&40) { command_stack.push(Box::new(&CancelDy)) }
    if !pressed_keys.contains(&37) && !pressed_keys.contains(&39) { command_stack.push(Box::new(&CancelDx)) }

    command_stack
  }
}

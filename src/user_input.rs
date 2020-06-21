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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Button {
  Up = 38,
  Right = 39,
  Down = 40,
  Left = 37,
}

#[derive(Debug)]
pub struct Commands {
  move_up: MoveUp,
  move_right: MoveRight,
  move_down: MoveDown,
  move_left: MoveLeft,
  cancel_dx: CancelDx,
  cancel_dy: CancelDy,
}

#[derive(Debug)]
pub struct InputHandler {
  pressed_keys: Vec<u32>,
  commands: Commands,
}

impl InputHandler {
  pub fn new() -> InputHandler {
    InputHandler {
      pressed_keys: Vec::new(),
      commands: Commands {
        move_up: MoveUp,
        move_right: MoveRight,
        move_down: MoveDown,
        move_left: MoveLeft,
        cancel_dx: CancelDx,
        cancel_dy: CancelDy,
      },
    }
  }
  pub fn handle_input(&mut self, pressed_keys: Vec<u32>) -> Vec<Box<&dyn Command>> {
    self.pressed_keys = pressed_keys;

    let mut commands: Vec<Box<&dyn Command>> = Vec::new();

    let up_pressed = self.is_pressed(Button::Up);
    let right_pressed = self.is_pressed(Button::Right);
    let down_pressed = self.is_pressed(Button::Down);
    let left_pressed = self.is_pressed(Button::Left);

    match up_pressed {
      true => commands.push(Box::new(&self.commands.move_up)),
      false if !down_pressed => commands.push(Box::new(&self.commands.cancel_dy)),
      _ => {}
    };

    match right_pressed {
      true => commands.push(Box::new(&self.commands.move_right)),
      false if !left_pressed => commands.push(Box::new(&self.commands.cancel_dx)),
      _ => {},
    };

    match down_pressed {
      true => commands.push(Box::new(&self.commands.move_down)),
      false if !up_pressed => commands.push(Box::new(&self.commands.cancel_dy)),
      _ => {}
    };

    match left_pressed {
      true => commands.push(Box::new(&self.commands.move_left)),
      false if !right_pressed => commands.push(Box::new(&self.commands.cancel_dx)),
      _ => {}
    };

    commands
  }
  fn is_pressed(&self, button: Button) -> bool {
    let btn = button as u32;
    self.pressed_keys.contains(&btn)
  }
}


pub trait GameActor {
  fn move_up(&self);
  fn move_down(&self);
  fn move_left(&self);
  fn move_right(&self);
}

pub trait Command {
  fn execute(&self, actor: &impl GameActor) where Self : Sized;
}


struct MoveUp;
impl Command for MoveUp {
  fn execute(&self, actor: &impl GameActor) {
    actor.move_up();
  }
}

struct MoveRight;
impl Command for MoveRight {
  fn execute(&self, actor: &impl GameActor) {
    actor.move_right();
  }
}

struct MoveDown;
impl Command for MoveDown {
  fn execute(&self, actor: &impl GameActor) {
    actor.move_down();
  }
}

struct MoveLeft;
impl Command for MoveLeft {
  fn execute(&self, actor: &impl GameActor) {
    actor.move_right();
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Button {
  Up = 38,
  Right = 39,
  Down = 40,
  Left = 37,
}

pub struct InputHandler {
  pressed_keys: Vec<u32>,
  move_up: MoveUp,
  move_right: MoveRight,
  move_down: MoveDown,
  move_left: MoveLeft,
}

impl InputHandler {
  pub fn new() -> InputHandler {
    InputHandler {
      pressed_keys: Vec::new(),
      move_up: MoveUp,
      move_right: MoveRight,
      move_down: MoveDown,
      move_left: MoveLeft
    }
  }
  pub fn handle_input(&mut self, pressed_keys: Vec<u32>) -> Vec<Box<&dyn Command>> {
    self.pressed_keys = pressed_keys;
    let mut commands: Vec<Box<&dyn Command>> = Vec::new();
    if self.is_pressed(Button::Up) { commands.push(Box::new(&self.move_up)) }
    if self.is_pressed(Button::Right) { commands.push(Box::new(&self.move_right)) }
    if self.is_pressed(Button::Down) { commands.push(Box::new(&self.move_down)) }
    if self.is_pressed(Button::Left) { commands.push(Box::new(&self.move_left)) }
    commands
  }
  fn is_pressed(&self, button: Button) -> bool {
    let btn = button as u32;
    self.pressed_keys.contains(&btn)
  }
}

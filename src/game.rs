
use wasm_bindgen::prelude::*;
use crate::utils;
use crate::physics;
use crate::universe::Universe;
use crate::sprite::{Sprite, command};
use crate::user_input::{
  UserInput,
  Button,
  // InputHandler,
  // parse_user_input,
  // interpret_user_input,
  // KeyCode,
};
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct SpriteConfig {
  width: u32,
  height: u32,
  center_row: u32,
  center_col: u32,
}


#[wasm_bindgen]
pub struct Game {
  universe: Universe,
  sprite: Sprite,
  user_input: UserInput,
}

#[wasm_bindgen]
impl Game {
  pub fn new(width: u32, height: u32, sprite_config: &JsValue) -> Game {
    utils::set_panic_hook();

    let sprite_config: SpriteConfig = sprite_config.into_serde().unwrap();

    let universe = Universe::new(width, height);

    let sprite: Sprite = Sprite::new(
      sprite_config.width,
      sprite_config.height,
      universe.get_index(sprite_config.center_row, sprite_config.center_col) as u32
    ).unwrap();

    Game { universe, sprite, user_input: UserInput::new(vec![]) }
  }
  pub fn tick(&mut self) {
    // save snapshot of universe cell state
    let previous = self.universe.snapshot();
    // set all universe cells to default state
    self.universe.blank_slate();
    // update sprite states
    physics::update_position(&mut self.sprite, &self.universe);
    
    // map sprite cell states onto universe cells
    self.universe.map_sprite(&self.sprite);
    // compare old universe cell states to new universe cell states,
    // returning only the indexes of the universe cells that changed
    self.universe.diff_frames(previous);
  }

  pub fn handle_user_input(&mut self, pressed_keys: &JsValue) {
    let pressed_keys: PressedKeys = pressed_keys.into_serde().unwrap();
    let pressed_keys = pressed_keys.0;
    self.user_input = UserInput::new(pressed_keys);

    let mut commands = Vec::new();

    let mut handle_arrow_keys = |
      b1: &Button,
      b2: &Button,
      c1: Box<dyn Command>,
      c2: Box<dyn Command>,
      cancel: Box<dyn Command>
    | {
      if self.user_input.has_all(vec![b1, b2]) {
        if self.user_input.has_order(b1, b2) {
          commands.push(c2);
        } else {
          commands.push(c1);
        }
      } else if self.user_input.has(b1) {
        commands.push(c1);
      } else if self.user_input.has(b2) {
        commands.push(c2);
      } else {
        commands.push(cancel);
      }
    };

    handle_arrow_keys(
      &Button::RightArrow,
      &Button::LeftArrow,
      Box::new(command::MoveRight),
      Box::new(command::MoveLeft),
      Box::new(command::CancelDx),
    );

    handle_arrow_keys(
      &Button::UpArrow,
      &Button::DownArrow,
      Box::new(command::MoveUp),
      Box::new(command::MoveDown),
      Box::new(command::CancelDy),
    );

    for command in commands {
      command.execute(&mut self.sprite);
    }
  }
  pub fn get_universe_cells_delta(&self) -> JsValue {
    JsValue::from_serde(self.universe.cells_delta()).unwrap()
  }
  pub fn get_universe_row_col(&self, idx: usize) -> JsValue {
    JsValue::from_serde(&self.universe.row_col(idx)).unwrap()
  }
  pub fn get_universe_cell_color_by_idx(&self, idx: usize) -> u8 {
    self.universe.get_cell_color_by_idx(idx)
  }
}


#[derive(Serialize, Deserialize, Debug)]
struct PressedKeys(Vec<u32>);


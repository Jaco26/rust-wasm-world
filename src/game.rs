
use wasm_bindgen::prelude::*;
use crate::utils;
use crate::physics;
use crate::universe::Universe;
use crate::sprite::Sprite;

const UP: u32 = 38;
const RIGHT: u32 = 39;
const DOWN: u32 = 40;
const LEFT: u32 = 37;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
  universe: Universe,
  sprite: Sprite,
  input_handler: UserInputHandler,
}

#[wasm_bindgen]
impl Game {
  pub fn new(width: u32, height: u32) -> Game {
    utils::set_panic_hook();

    let universe = Universe::new(width, height);

    let sprite: Sprite = Sprite::new(3, 5, universe.get_index(10, 10) as u32).unwrap();

    Game { universe, sprite, input_handler: UserInputHandler::new() }
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
    self.input_handler.set_pressed_keys(pressed_keys.into_serde().unwrap());

    let up_pressed = self.input_handler.is_pressed(UP);
    let right_pressed = self.input_handler.is_pressed(RIGHT);
    let down_pressed = self.input_handler.is_pressed(DOWN);
    let left_pressed = self.input_handler.is_pressed(LEFT);

    if up_pressed {
      self.sprite.set_dy(-1);
    } else if !down_pressed {
      self.sprite.set_dy(0);
    }

    if right_pressed {
      self.sprite.set_dx(1);
    } else if !left_pressed {
      self.sprite.set_dx(0);
    }

    if down_pressed {
      self.sprite.set_dy(1);
    } else if !up_pressed {
      self.sprite.set_dy(0);
    }

    if left_pressed {
      self.sprite.set_dx(-1);
    } else if !right_pressed {
      self.sprite.set_dx(0);
    }
    // log!("self.input_handler::is_pressed(38): {}", self.input_handler.is_pressed(38, &pressed_keys));
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


#[derive(Debug)]
struct UserInputHandler {
  pressed_keys: Vec<u32>,
}

impl UserInputHandler {
  pub fn new() -> UserInputHandler {
    UserInputHandler { pressed_keys: Vec::new() }
  }
  pub fn set_pressed_keys(&mut self, pressed_keys: PressedKeys) {
    self.pressed_keys = pressed_keys.0;
  }
  pub fn is_pressed(&self, key_code: u32) -> bool {
    self.pressed_keys.contains(&key_code)
  }
  // pub fn is_pressed(&self, key_code: u32, user_input: &PressedKeys) -> bool {
  //   user_input.0.contains(&key_code)
  // }
}
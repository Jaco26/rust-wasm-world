use wasm_bindgen::prelude::*;
use crate::utils;
use crate::physics;
use crate::universe::Universe;
use crate::sprite::Sprite;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
  universe: Universe,
  sprite: Sprite,
}

#[wasm_bindgen]
impl Game {
  pub fn new(width: u32, height: u32) -> Game {
    utils::set_panic_hook();

    let universe = Universe::new(width, height);

    let sprite: Sprite = Sprite::new(3, 3, universe.get_index(10, 10) as u32).unwrap();

    Game { universe, sprite }
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

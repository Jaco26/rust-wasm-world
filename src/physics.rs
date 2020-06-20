use wasm_bindgen::prelude::*;
use crate::sprite::Sprite;
use crate::universe::Universe;


#[wasm_bindgen]
pub fn update_position(sprite: &mut Sprite, universe: &Universe) {
  let (row, col) = universe.get_row_and_col(sprite.center_index());
  let new_row = row + sprite.dy();
  let new_col = col + sprite.dx();
  let center_idx = universe.get_index(new_row, new_col);
  sprite.set_center_idx(center_idx);
  sprite.center_self();
}
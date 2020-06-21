use wasm_bindgen::prelude::*;
use crate::sprite::Sprite;
use crate::universe::Universe;


#[wasm_bindgen]
pub fn update_position(sprite: &mut Sprite, universe: &Universe) {
  let (center_row, center_col) = universe.get_row_and_col(sprite.center_index());
  let mut new_center_row = center_row as i32 + sprite.dy();
  let mut new_center_col = center_col as i32 + sprite.dx();

  if new_center_row + sprite.height() as i32 / 2 >= universe.height() as i32 {
    new_center_row = 0;
  } else if new_center_row - sprite.height() as i32 / 2 <= 0 {
    new_center_row = universe.height() as i32 - 1;
  }

  if new_center_col >= universe.width() as i32 {
    new_center_col = 0;
  } else if new_center_col <= 0 {
    new_center_col = universe.width() as i32 - 1;
  }

  // log!("new_center_row, new_center_col: {}, {}", new_center_row, new_center_col);

  let new_center_idx = universe.get_index(new_center_row as u32, new_center_col as u32);

  sprite.set_center_idx(new_center_idx);
  sprite.center_self(universe);
}



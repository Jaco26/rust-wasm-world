pub mod commands;
pub mod command_dispatchers;


use wasm_bindgen::prelude::*;
use crate::prelude::GameActor;
use crate::universe::Universe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteCell {
  pub clr_idx: u8,
  pub pos_idx: usize,
}

impl SpriteCell {
  pub fn new(clr_idx: u8, pos_idx: usize) -> SpriteCell {
    SpriteCell { clr_idx, pos_idx }
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
/// A `Sprite` occupies a subset of cells in a `Universe`
pub struct Sprite {
  width: u32,
  height: u32,
  center_index: usize,
  body: Vec<SpriteCell>,
  dx: i32,
  dy: i32,
  animation_frame: u8,
}

impl Sprite {
  pub fn new(width: u32, height: u32, center_index: u32) -> Result<Sprite, JsValue> {
    if width * height % 2 == 0 {
      return Err(JsValue::from_str("A sprite must have an odd number of cells"));
    }
    let body_start = center_index - (width * height) / 2;
    let body_end = center_index + (width * height) / 2;
    Ok(Sprite {
      animation_frame: 0,
      dx: 0,
      dy: 0,
      width,
      height,
      center_index: center_index as usize,
      body: (body_start..body_end).map(|x| SpriteCell::new(9, x as usize)).collect()
    })
  }
  pub fn width(&self) -> u32 {
    self.width
  }
  pub fn height(&self) -> u32 {
    self.height
  }
  pub fn dy(&self) -> i32 {
    self.dy
  }
  pub fn dx(&self) -> i32 {
    self.dx
  }
  pub fn center_index(&self) -> usize {
    self.center_index
  }
  pub fn set_center_idx(&mut self, idx: usize) {
    self.center_index = idx;
  }
  fn get_body_cell_indeces(&self, universe: &Universe) -> Vec<usize> {
    let (center_row, center_col) = universe.get_row_and_col(self.center_index);
    let cols_range = (center_col - self.width / 2)..(center_col + self.width / 2) + 1;
    let rows_range = (center_row - self.height / 2)..(center_row + self.height / 2) + 1;
    let mut rv = Vec::new();
    for row in rows_range {
      for col in cols_range.clone() {
        let row = row % universe.height();
        let col = col % universe.width();
        rv.push(universe.get_index(row, col));
      }
    }
    rv
  }
  /// Update the `Sprite.body` cell `idx`s so that the body is centered on the `Sprite.center_index`
  pub fn center_self(&mut self, universe: &Universe) {
    self.body = self.get_body_cell_indeces(universe)
      .iter()
      .map(|&x| SpriteCell::new(9, x))
      .collect();
  }

  pub fn update(&mut self) {
    
  }
}


impl Sprite {
  pub fn body(&self) -> &Vec<SpriteCell> {
    &self.body
  }
}



impl GameActor for Sprite {
  fn move_up(&mut self) {
    self.dy = -1;
  }
  fn move_right(&mut self) {
    self.dx = 1;
  }
  fn move_down(&mut self) {
    self.dy = 1;
  }
  fn move_left(&mut self) {
    self.dx = -1;
  }
  fn cancel_dx(&mut self) {
    self.dx = 0;
  }
  fn cancel_dy(&mut self) {
    self.dy = 0;
  }
}
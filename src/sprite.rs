use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteCell {
  pub clr_idx: u8,
  pub pos_idx: usize,
}

#[wasm_bindgen]
impl SpriteCell {
  pub fn new(clr_idx: u8, pos_idx: usize) -> SpriteCell {
    SpriteCell { clr_idx, pos_idx }
  }
}


#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A `Sprite` occupies a subset of cells in a `Universe`
pub struct Sprite {
  width: u32,
  height: u32,
  center_index: usize,
  body: Vec<SpriteCell>,
  dx: u32,
  dy: u32,
}

#[wasm_bindgen]
impl Sprite {
  pub fn new(width: u32, height: u32, center_index: u32) -> Result<Sprite, JsValue> {
    if width * height % 2 == 0 {
      return Err(JsValue::from_str("A sprite must have an odd number of cells"));
    }
    let body_start = center_index - (width * height) / 2;
    let body_end = center_index + (width * height) / 2;
    Ok(Sprite {
      dx: 1,
      dy: 0,
      width,
      height,
      center_index: center_index as usize,
      body: (body_start..body_end).map(|x| SpriteCell::new(2, x as usize)).collect()
    })
  }
  pub fn dy(&self) -> u32 {
    self.dy
  }
  pub fn set_dy(&mut self, dy: u32) {
    self.dy = dy;
  }
  pub fn dx(&self) -> u32 {
    self.dx
  }
  pub fn set_dx(&mut self, dx: u32) {
    self.dx += dx;
  }
  pub fn center_index(&self) -> usize {
    self.center_index
  }
  pub fn set_center_idx(&mut self, idx: usize) {
    self.center_index = idx;
  }
  /// Get the range of `Universe` cell indexes that the `Sprite.body` currently occupies
  fn get_body_range(&self) -> std::ops::Range<usize> {
    let center = self.center_index;
    let len_body_half = ((self.width * self.height) / 2) as usize;
    (center - len_body_half)..(center + len_body_half) + 1
  }
  /// Update the `Sprite.body` cell `idx`s so that the body is centered on the `Sprite.center_index`
  pub fn center_self(&mut self) {
    self.body = self.get_body_range().map(|x| SpriteCell::new(2, x)).collect();
  }
  /// Accept a
  fn apply_color_frame(&mut self, frame: Vec<u8>) {
    for (i, body_idx) in self.get_body_range().enumerate() {
      self.body[body_idx].clr_idx = frame[i];
    }
  }
}


impl Sprite {
  pub fn body(&self) -> &Vec<SpriteCell> {
    &self.body
  }
}
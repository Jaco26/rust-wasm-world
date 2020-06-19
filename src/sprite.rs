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
pub struct SpriteCell {
  color_index: u8,
  position_index: usize,
}

#[wasm_bindgen]
impl SpriteCell {
  pub fn new(color_index: u8, position_index: usize) -> SpriteCell {
    SpriteCell { color_index, position_index }
  }
  pub fn update_color(&mut self, color_index: u8) {
    self.color_index = color_index;
  }
  pub fn update_position(&mut self, position_index: usize) {
    self.position_index = position_index;
  }
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct Sprite {
  universe_width: u32,
  universe_height: u32,
  width: u32,
  height: u32,
  center_index: usize,
  body: Vec<usize>,
}

#[wasm_bindgen]
impl Sprite {
  pub fn new(width: u32, height: u32, center_index: u32) -> Sprite {
    let body_start = center_index - (width * height) / 2;
    let body_end = center_index + (width * height) / 2;
    Sprite {
      width,
      height,
      center_index: center_index as usize,
      body: (body_start..body_end).map(|x| x as usize).collect()
    }
  }
  fn center_self(&mut self) {
    let center_index = self.center_index as u32;
    let body_start = center_index - (self.width * self.height) / 2;
    let body_end = center_index + (self.width * self.height) / 2;
    self.body = (body_start..body_end).map(|x| x as usize).collect();
  }
  fn move_up(&mut self) {

  }
  fn move_right(&mut self) {
    
  }
  fn move_down(&mut self) {

  }
  fn move_left(&mut self) {

  }
  pub fn update_position(&self, direction: Direction) {
    match direction {
      Direction::Up => self.move_up(),
      Direction::Right => self.move_up(),
      Direction::Down => self.move_down(),
      Direction::Left => self.move_up(),
    };
  }
}

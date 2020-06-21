use wasm_bindgen::prelude::*;
use crate::sprite::Sprite;


#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
  color_index: u8, // used as index to map to color dictionary in javascript
}

impl Cell {
  fn new() -> Cell {
    Cell { color_index: 0 }
  }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
  height: u32,
  width: u32,
  cells: Vec<Cell>,
  cells_delta: Vec<usize>,
}

#[wasm_bindgen]
impl Universe {
  pub fn get_index(&self, row: u32, col: u32) -> usize {
    if row > self.height {
      panic!("row {} is out of range. row must be between 0 and {}", row, self.height)
    }
    if col > self.width {
      panic!("col {} is out of range. col must be between 0 and {}", col, self.width);
    }
    (self.width * row + col ) as usize
  }

  pub fn new(width: u32, height: u32) -> Universe {
    Universe {
      width,
      height,
      cells: (0..width * height).map(|_x| Cell::new()).collect(),
      cells_delta: (0..width * height).map(|x| x as usize).collect()
    }
  }

  pub fn width(&self) -> u32 {
    self.width
  }
  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.blank_slate();
  }
  pub fn height(&self) -> u32 {
    self.height
  }
  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.blank_slate();
  }
  pub fn get_cell_color(&self, row: u32, col: u32) -> u8 {
    self.cells[self.get_index(row, col)].color_index
  }
  pub fn get_cell_color_by_idx(&self, idx: usize) -> u8 {
    self.cells[idx].color_index
  }

  pub fn set_cell_color(&mut self, row: u32, col: u32, color_index: u8) {
    let idx = self.get_index(row, col);
    self.cells[idx].color_index = color_index;
  }

}

impl Universe {
  pub fn get_row_and_col(&self, idx: usize) -> (u32, u32) {
    let idx = idx as u32;
    let row = idx / self.width;
    let col = idx % self.width;
    (row, col)
  }
  pub fn row_col(&self, idx: usize) -> (u32, u32) {
    let idx = idx as u32;
    let row = idx / self.width;
    let col = idx % self.width;
    (row, col)
  }
}

// NEW
impl Universe {
  pub fn snapshot(&self) -> Vec<Cell> {
    self.cells.clone()
  }
  pub fn blank_slate(&mut self) {
    self.cells = (0..self.width * self.height).map(|_x| Cell::new()).collect();
  }
  pub fn map_sprite(&mut self, sprite: &Sprite) {
    for c in sprite.body() {
      self.cells[c.pos_idx].color_index = c.clr_idx;
    }
  }
  pub fn diff_frames(&mut self, snapshot: Vec<Cell>) {
    let idxs = (0..self.cells.len()).map(|x| x);
    let zip = snapshot.iter().zip(self.cells.iter()).zip(idxs);
    self.cells_delta = zip.fold(Vec::<usize>::new(), |mut acc, ((old, new), idx)| {
      if old.color_index != new.color_index {
        acc.push(idx);
      }
      acc
    })
  }
  pub fn cells_delta(&self) -> &Vec<usize> {
    &self.cells_delta
  }
}
/// Code for main interface used by the javascript code
use wasm_bindgen::prelude::*;
use crate::utils;


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Cell {
  color_index: u8, // used as index to map to color dictionary in javascript
}

impl Cell {
  fn new() -> Cell {
    utils::set_panic_hook();
    Cell { color_index: 0 }
  }
}

#[wasm_bindgen]
pub struct Universe {
  height: u32,
  width: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  fn get_index(&self, row: u32, col: u32) -> usize {
    if row > self.height {
      panic!("row {} is out of range. row must be between 0 and {}", row, self.height)
    }
    if col > self.width {
      panic!("col {} is out of range. col must be between 0 and {}", col, self.width);

    }
    (self.width * row + col ) as usize
  }
  fn generate_cells(&mut self) {
    self.cells = (0..self.width * self.height).map(|_x| Cell::new()).collect();
  }
  pub fn new(width: u32, height: u32) -> Universe {
    Universe { width, height, cells: (0..width * height).map(|_x| Cell::new()).collect() }
  }
  pub fn width(&self) -> u32 {
    self.width
  }
  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.generate_cells();
  }
  pub fn height(&self) -> u32 {
    self.height
  }
  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.generate_cells();
  }
  pub fn get_cell_color(&self, row: u32, col: u32) -> u8 {
    self.cells[self.get_index(row, col)].color_index
  }
  pub fn set_cell_color(&mut self, row: u32, col: u32, color_index: u8) {
    let idx = self.get_index(row, col);
    self.cells[idx].color_index = color_index;
  }
}
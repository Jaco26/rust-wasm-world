import { Game } from 'pixel-person';
import * as c from './constants/colors'
import color from './utils/color'

// const WIDTH = 128
// const HEIGHT = 80

// const WIDTH = 172
// const HEIGHT = 128

const WIDTH = 230
const HEIGHT = 172
const CELL_SIZE = 4

/** @type {HTMLCanvasElement} */
const layer1 = document.getElementById('layer-1')

layer1.width = WIDTH * CELL_SIZE
layer1.height = HEIGHT * CELL_SIZE

const layer1Ctx = layer1.getContext('2d')

// const universe = Universe.new(WIDTH, HEIGHT)

// universe.set_cell_color(2, 0, color.getIdx(c.GRAY_DARK_2))
// universe.set_cell_color(2, 1, color.getIdx(c.GRAY_DARK_1))
// universe.set_cell_color(2, 2, color.getIdx(c.GRAY))
// universe.set_cell_color(2, 3, color.getIdx(c.GRAY_LIGHT_1))
// universe.set_cell_color(2, 4, color.getIdx(c.GRAY_LIGHT_2))

// universe.set_cell_color(3, 0, color.getIdx(c.BLUE_DARK_2))
// universe.set_cell_color(3, 1, color.getIdx(c.BLUE_DARK_1))
// universe.set_cell_color(3, 2, color.getIdx(c.BLUE))
// universe.set_cell_color(3, 3, color.getIdx(c.BLUE_LIGHT_1))
// universe.set_cell_color(3, 4, color.getIdx(c.BLUE_LIGHT_2))

// universe.set_cell_color(4, 0, color.getIdx(c.RED_DARK_2))
// universe.set_cell_color(4, 1, color.getIdx(c.RED_DARK_1))
// universe.set_cell_color(4, 2, color.getIdx(c.RED))
// universe.set_cell_color(4, 3, color.getIdx(c.RED_LIGHT_1))
// universe.set_cell_color(4, 4, color.getIdx(c.RED_LIGHT_2))

// universe.set_cell_color(5, 0, color.getIdx(c.GREEN_DARK_2))
// universe.set_cell_color(5, 1, color.getIdx(c.GREEN_DARK_1))
// universe.set_cell_color(5, 2, color.getIdx(c.GREEN))
// universe.set_cell_color(5, 3, color.getIdx(c.GREEN_LIGHT_1))
// universe.set_cell_color(5, 4, color.getIdx(c.GREEN_LIGHT_2))



const game = Game.new(WIDTH, HEIGHT)

function drawCells() {
  const cellsDelta = game.get_universe_cells_delta();
  for (let c = 0; c < cellsDelta.length; c++) {
    const cellIdx = cellsDelta[c]
    const [row, col] = game.get_universe_row_col(cellIdx)
    layer1Ctx.fillStyle = color.getClr(game.get_universe_cell_color_by_idx(cellIdx))
    layer1Ctx.fillRect(
      col * CELL_SIZE + 1,
      row * CELL_SIZE + 1,
      CELL_SIZE - 1,
      CELL_SIZE - 1,
    )
  }
}

function animationLoop() {
  drawCells()
  game.tick()
  requestAnimationFrame(animationLoop)
}

function drawOnce() {
  drawCells()
  game.tick()
  drawCells()
}
// drawOnce()
animationLoop()



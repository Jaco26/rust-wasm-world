import { Game } from 'pixel-person';
import * as c from './constants/colors'
import color from './utils/color'
import InputHandler from './utils/input'

const UNIVERSE_SIZES = [
  [60, 40, 12],
  [80, 56, 10],
  [128, 80, 8],
  [172, 128, 5],
  [256, 172, 4],
  [512, 342, 2],
]

const [WIDTH, HEIGHT, CELL_SIZE] = UNIVERSE_SIZES[3]


/** @type {HTMLCanvasElement} */
const layer1 = document.getElementById('layer-1')

layer1.width = WIDTH * CELL_SIZE
layer1.height = HEIGHT * CELL_SIZE

const layer1Ctx = layer1.getContext('2d')


const inputHandler = new InputHandler()

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
  game.handle_user_input(inputHandler.getPressedKeys())
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



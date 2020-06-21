

export default class InputHandler {
  constructor() {
    this._pressedKeys = new Set()

    window.addEventListener('keydown', e => {
      this._pressedKeys.add(e.keyCode)
    })

    window.addEventListener('keyup', e => {
      this._pressedKeys.delete(e.keyCode)
    })
  }

  getPressedKeys() {
    return [...this._pressedKeys]
  }
}
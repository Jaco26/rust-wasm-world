import * as c from '../constants/colors'

class ColorMap {
  constructor() {
    this._byKey = {}
    this._byIndex = []
  }
  set(key, red, green, blue, alpha) {
    const data = {
      idx: this._byIndex.length,
      clr: typeof alpha === 'number' ? `rgba(${red},${green},${blue},${alpha})` : `rgb(${red},${green},${blue})`
    }
    this._byKey[key] = data
    this._byIndex.push(data)
  }
  getClr(index) {
    return (this._byIndex[index] || this._byIndex[0] || {}).clr
  }
  getIdx(key) {
    return (this._byKey[key] || {}).idx
  }
}

const cm = new ColorMap()
cm.set(c.WHITE, 255, 255, 255)
cm.set(c.BLACK, 0, 0, 0)
cm.set(c.GRAY, 150, 150, 150)
cm.set(c.GRAY_DARK_1, 100, 100, 100)
cm.set(c.GRAY_DARK_2, 50, 50, 50)
cm.set(c.GRAY_LIGHT_1, 150, 150, 150, .6)
cm.set(c.GRAY_LIGHT_2, 150, 150, 150, .4)
cm.set(c.BLUE, 0, 0, 255)
cm.set(c.BLUE_DARK_1, 0, 0, 200)
cm.set(c.BLUE_DARK_2, 0, 0, 150)
cm.set(c.BLUE_LIGHT_1, 0, 0, 255, .6)
cm.set(c.BLUE_LIGHT_2, 0, 0, 255, .4)

cm.set(c.RED, 255, 0, 0)
cm.set(c.RED_DARK_1, 200, 0, 0)
cm.set(c.RED_DARK_2, 150, 0, 0)
cm.set(c.RED_LIGHT_1, 255, 0, 0, .6)
cm.set(c.RED_LIGHT_2, 255, 0, 0, .4)

cm.set(c.GREEN, 0, 255, 0)
cm.set(c.GREEN_DARK_1, 0, 200, 0)
cm.set(c.GREEN_DARK_2, 0, 150, 0)
cm.set(c.GREEN_LIGHT_1, 0, 255, 0, .6)
cm.set(c.GREEN_LIGHT_2, 0, 255, 0, .4)


export default cm
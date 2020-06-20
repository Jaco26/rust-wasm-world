

export function range(low, high) {
  if (!high) {
    return [...Array(low).keys()]
  }
  return [...Array(high - low).keys()]
}



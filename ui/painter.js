export class Painter {
  constructor(row, col) {
    this.row = row;
    this.col = col;
    this.root = document.getElementsByClassName('snake-painter')[0]
    this.map = new Array(row).fill(0).map(x => new Array(col).fill(0))
  }


  draw(snake, food) {
    const fragment = document.createDocumentFragment()

    this.clear();
    snake.get_body().forEach(([x, y], i) => {
      this.map[x][y] = i === 0 ? 2 : 1
    })
    this.map[food[0]][food[1]] = 3
    this.map.forEach((s) => {
      s.forEach((x, j) => {
        const wrapperDom = document.createElement('div')
        switch (x) {
          case 0:
            wrapperDom.innerHTML = ``
            break
          case 1:
            wrapperDom.innerHTML = '<div class="snake-body"></div>'
            break
          case 2:
            wrapperDom.innerHTML = '<div class="snake-body snake-head"></div>'
            break
          case 3:
            wrapperDom.innerHTML = '<div class="snake-food"></div>'
        }

        fragment.appendChild(wrapperDom)
      })
    })

    this.root.innerHTML = ``;

    this.root.appendChild(fragment)
  }

  clear() {
    this.map.forEach(s => {
      s.forEach((x, i) => {
        s[i] = 0
      })
    })
  }
}
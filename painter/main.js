import './style.css'
import init, { Snake } from 'snake-wasm'

function sleep(time) {
  return new Promise(r => setTimeout(r, time))
}

function genFood(row, col) {
  return [
    parseInt(Math.random() * row),
    parseInt(Math.random() * col)
  ]
}

class Painter {
  constructor(row, col) {
    this.row = row;
    this.col = col;
    this.root = document.getElementById('app')
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

    this.root.innerHTML = '';

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

const row = 30
const col = 40

const painter = new Painter(row, col)

init().then(async () => {
  const snake = Snake.new(row, col);
  let food = genFood(row, col)
  let direction = "RIGHT"
  let updating = false

  function update() {
    if (updating) {
      return
    }
    updating = true
    painter.draw(snake, food)

    snake.update_pos(direction)

    if (snake.eat(food[0], food[1])) {
      food = genFood(row, col)
    }
    updating = false
  }

  document.addEventListener('keydown', e => {
    const nextDirection = e.code.replace('Arrow', '').toUpperCase()
    const directions = `${direction}${nextDirection}`

    if ((/UP/.test(directions) && /DOWN/.test(directions)) || (/LEFT/.test(directions) && /RIGHT/.test(directions))) {
      return
    }

    direction = nextDirection

    update()
  }, false)

  while (!snake.is_hit()) {
    await sleep(200)
    update()
  }

  await sleep(200)
  alert("You Lose!! Fresh to restart..")
})
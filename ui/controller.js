import { Snake } from 'snake-wasm'
import { Painter } from './painter';

export class Controller {
  constructor(row, col) {
    this.row = row
    this.col = col
    this.snake = Snake.new(row, col);
    this.painter = new Painter(row, col);
    this.food = this.genFood(row, col)
    this.direction = "RIGHT"
    this.updating = false
    this.updateId = setInterval(this.move.bind(this), 200)
    this.isPause = true
  }

  move({ force = false, nextDirection } = {}) {
    if (!force && (this.isPause || this.updating)) {
      return
    }

    const directions = `${this.direction}${nextDirection}`

    if (
      this.direction === nextDirection ||
      (/UP/.test(directions) && /DOWN/.test(directions)) || (/LEFT/.test(directions) && /RIGHT/.test(directions))) {
      return
    }

    if (this.snake.is_hit()) {
      clearInterval(this.updateId)
      alert("You Lose!! Fresh to restart..")
      return
    }

    nextDirection = nextDirection || this.direction

    this.updating = true
    this.painter.draw(this.snake, this.food)

    this.snake.update_pos(nextDirection)

    this.direction = nextDirection

    if (this.snake.eat(this.food[0], this.food[1])) {
      this.food = this.genFood()
    }

    this.updating = false
  }

  pauseOrResume() {
    this.isPause = !this.isPause
  }

  genFood() {
    return [
      parseInt(Math.random() * this.row),
      parseInt(Math.random() * this.col)
    ]
  }
}
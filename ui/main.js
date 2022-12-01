import './style.css'
import init from 'snake-wasm'
import { Painter } from './painter'
import { Controller } from './controller'

function sleep(time) {
  return new Promise(r => setTimeout(r, time))
}

const row = 30
const col = 40
const isRobot = !!import.meta.env['VITE_ROBOT']

  ; (async () => {
    await init()

    const controller = new Controller(row, col, isRobot)

    document.addEventListener('keydown', e => {
      if (e.code === 'Space') {
        controller.pauseOrResume()
      }

      if (!isRobot && /Arrow/.test(e.code)) {
        const nextDirection = e.code.replace('Arrow', '').toUpperCase()

        controller.move({ nextDirection })
      }
    }, false)

    controller[isRobot ? 'autoMove' : 'move']({ force: true })
  })()



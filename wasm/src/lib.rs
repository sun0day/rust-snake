mod utils;

use js_sys::Array;
use std::option::Option;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Snake {
    area: (usize, usize),
    body: Vec<(usize, usize)>,
    last_tail: (usize, usize),
}

#[wasm_bindgen]
impl Snake {
    pub fn new(rows: usize, cols: usize) -> Snake {
        let head = (rows / 2, cols / 2);
        let mut body = vec![head];

        for i in 1..5 {
            body.push((head.0, head.1 - i))
        }

        let last_tail = body[body.len() - 1];

        Snake {
            area: (rows, cols),
            body: body,
            last_tail: last_tail,
        }
    }

    pub fn move_pos(&mut self, direction: String) {
        let mut last = self.body[0];
        let length = self.body.len();

        self.body[0] = match direction.as_str() {
            "UP" => (last.0 - 1, last.1),
            "DOWN" => (last.0 + 1, last.1),
            "LEFT" => (last.0, last.1 - 1),
            "RIGHT" => (last.0, last.1 + 1),
            _ => return,
        };

        self.last_tail = self.body[length - 1];

        for i in 1..length {
            let tmp = self.body[i];
            self.body[i] = last;
            last = tmp
        }
    }

    pub fn eat(&mut self, foodX: usize, foodY: usize) -> bool {
        let head = self.body[0];

        if head.0 == foodX && head.1 == foodY {
            self.body.push(self.last_tail);
            return true;
        }

        false
    }

    pub fn is_hit(&self) -> bool {
        let head = self.body[0];

        if head.0 < 0 || head.0 >= self.area.0 || head.1 < 0 || head.1 >= self.area.1 {
            return true;
        }

        for p in &self.body[1..] {
            if head.0 == p.0 && head.1 == p.1 {
                return true;
            }
        }

        false
    }

    pub fn get_body(&self) -> Array {
        let mut arr = Array::new();

        for p in &self.body {
            let mut pa = Array::new();
            pa.push(&JsValue::from(p.0));
            pa.push(&JsValue::from(p.1));

            arr.push(&pa);
        }

        arr
    }

    pub fn auto_move_pos(&mut self, foodX: usize, foodY: usize) {
        let direction = self.get_next_move_to_food(foodX, foodY);

        self.move_pos(direction);
    }

    fn get_next_move_to_food(&mut self, foodX: usize, foodY: usize) -> String {
        utils::set_panic_hook();
        let head = self.body[0];
        let mut pos_queue = vec![head];
        let mut pos_map = self.init_pos_map();
        let mut pos = head;

        let update_pos_map = |pos: (usize, usize),
                              next_pos: (usize, usize),
                              pos_queue: &mut Vec<(usize, usize)>,
                              pos_map: &mut Vec<Vec<[usize; 3]>>| {
            if next_pos.0 < 0
                || next_pos.0 >= self.area.0
                || next_pos.1 < 0
                || next_pos.1 >= self.area.1
            {
                return;
            }
            if pos_map[next_pos.0][next_pos.1][2] == 0 {
                pos_map[next_pos.0][next_pos.1] = [pos.0, pos.1, 1];
                pos_queue.push(next_pos);
            }
        };

        while pos_queue.len() > 0 {
            pos = pos_queue.remove(0);
            if pos.0 == foodX && pos.1 == foodY {
                break;
            }

            update_pos_map(pos, (pos.0 + 1, pos.1), &mut pos_queue, &mut pos_map);
            update_pos_map(pos, (pos.0 - 1, pos.1), &mut pos_queue, &mut pos_map);
            update_pos_map(pos, (pos.0, pos.1 + 1), &mut pos_queue, &mut pos_map);
            update_pos_map(pos, (pos.0, pos.1 - 1), &mut pos_queue, &mut pos_map);
        }

        while pos_map[pos.0][pos.1][0] != head.0 || pos_map[pos.0][pos.1][1] != head.1 {
            pos = (pos_map[pos.0][pos.1][0], pos_map[pos.0][pos.1][1]);
        }

        String::from(if pos.0 < head.0 {
            "UP"
        } else if pos.0 > head.0 {
            "DOWN"
        } else if pos.1 < head.1 {
            "LEFT"
        } else if pos.1 > head.1 {
            "RIGHT"
        } else {
            "DOWN"
        })
    }

    fn init_pos_map(&self) -> Vec<Vec<[usize; 3]>> {
        let head = self.body[0];
        let mut map = Vec::with_capacity(self.area.0);

        for i in 0..self.area.0 {
            map.push(vec![[head.0, head.1, 0]; self.area.1]);
        }

        for body in &self.body {
            map[body.0][body.1] = [head.0, head.1, 1];
        }

        map
    }
}

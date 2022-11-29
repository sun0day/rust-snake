mod utils;

use js_sys::Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Snake {
    area: (u32, u32),
    body: Vec<(u32, u32)>,
    last_tail: (u32, u32),
}

#[wasm_bindgen]
impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head = (width / 2, height / 2);
        let mut body = vec![head];

        for i in 1..5 {
            body.push((head.0, head.1 - i))
        }

        let last_tail = body[body.len() - 1];

        Snake {
            area: (width, height),
            body: body,
            last_tail: last_tail,
        }
    }

    pub fn update_pos(&mut self, direction: String) {
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

    pub fn eat(&mut self, foodX: u32, foodY: u32) -> bool {
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
}

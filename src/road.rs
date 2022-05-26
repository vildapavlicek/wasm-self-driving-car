use js_sys::Array;
use std::ops::Neg;
use wasm_bindgen::prelude::*;

// if set too high, lanes won't be drawn
const INFINITY: f64 = 10_000 as f64;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Border {
    top_x: f64,
    top_y: f64,
    bottom_x: f64,
    bottom_y: f64,
}

#[wasm_bindgen]
impl Border {
    pub fn new(top_x: f64, top_y: f64, bottom_x: f64, bottom_y: f64) -> Border {
        Border {
            top_x,
            top_y,
            bottom_x,
            bottom_y,
        }
    }

    pub fn top_x(&self) -> f64 {
        self.top_x
    }

    pub fn top_y(&self) -> f64 {
        self.top_y
    }

    pub fn bottom_x(&self) -> f64 {
        self.bottom_x
    }

    pub fn bottom_y(&self) -> f64 {
        self.bottom_y
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Road {
    x: f64,
    width: f64,
    lane_count: i32,
    left: f64,
    right: f64,

    top: f64,
    bottom: f64,
    borders: Vec<Border>,
}

#[wasm_bindgen]
impl Road {
    pub fn new(x: f64, width: f64, lane_count: i32) -> Self {
        let left = x - width / 2.;
        let right = x + width / 2.;

        let top = INFINITY.neg();
        let bottom = INFINITY;

        let left_border = Border::new(left, top, left, bottom);
        let right_border = Border::new(right, top, right, bottom);

        let borders = vec![left_border, right_border];

        Self {
            x,
            width,
            lane_count,
            left,
            right,
            top,
            bottom,
            borders,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn lane_count(&self) -> i32 {
        self.lane_count
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn borders(&self) -> Array {
        self.borders.iter().copied().map(JsValue::from).collect()
    }

    pub fn lane_center(&self, lane_index: i32) -> f64 {
        let lane_width = self.width / self.lane_count as f64;
        self.left + lane_width / 2. + (lane_index.min(self.lane_count - 1) as f64) * lane_width
    }
}

/*
class Road {
    constructor(x, width, laneCount = 3) {
        this.x = x;
        this.width = width;
        this.laneCount = laneCount;

        this.left = x - width / 2;
        this.right = x + width / 2;
        const infinity = 10000;

        this.top = -infinity;
        this.bottom = infinity;

        const topLeft = {x: this.left, y: this.top };
        const bottomLeft = {x: this.left, y: this.bottom };
        const topRight = {x: this.right, y: this.top };
        const bottomRight = {x: this.right, y: this.bottom };

        this.borders = [
            [topLeft, bottomLeft],
            [topRight, bottomRight]
        ];
    }

    getLaneCenter(laneIndex) {
        const laneWidth = this.width / this.laneCount;
        return this.left + laneWidth / 2 +
            Math.min(laneIndex, this.laneCount - 1) * laneWidth;
    }

    draw(ctx) {
        ctx.lineWidth = 5;
        ctx.strokeStyle = "white";

        for (let i = 1; i <= this.laneCount - 1; i++) {
            const x = lerp(this.left, this.right, i / this.laneCount);

            ctx.setLineDash([20, 20]);

            ctx.beginPath();
            ctx.moveTo(x, this.top);
            ctx.lineTo(x, this.bottom);
            ctx.stroke();
        }

        ctx.setLineDash([]);
        this.borders.forEach(border => {
            ctx.beginPath();
            ctx.moveTo(border[0].x, border[0].y);
            ctx.lineTo(border[1].x, border[1].y);
            ctx.stroke();
        })
    }
}

*/

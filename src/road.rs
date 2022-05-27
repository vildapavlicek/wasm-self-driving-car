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
    dash_line: Array,
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
        let dash_line = Array::new();
        // dash_line.fill(&JsValue::from(20), 0, 2);
        dash_line.push(&JsValue::from(20));
        dash_line.push(&JsValue::from(20));

        crate::log!("{:?}", dash_line);

        Self {
            x,
            width,
            lane_count,
            left,
            right,
            top,
            bottom,
            borders,
            dash_line,
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

    pub fn draw(&self, ctx: web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(5.);
        ctx.set_stroke_style(&JsValue::from_str("white"));

        for i in 1..self.lane_count {
            let inner_x =
                crate::utils::lerp(self.left, self.right, i as f64 / self.lane_count as f64);
            let _ = ctx.set_line_dash(self.dash_line.as_ref());

            ctx.begin_path();
            ctx.move_to(inner_x, self.top);
            ctx.line_to(inner_x, self.bottom);
            ctx.stroke();
        }

        let _ = ctx.set_line_dash(Array::new().as_ref());

        self.borders.iter().for_each(|b| {
            ctx.begin_path();
            ctx.move_to(b.top_x(), b.top_y());
            ctx.line_to(b.bottom_x(), b.bottom_y());
            ctx.stroke();
        });
    }
}

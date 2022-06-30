use crate::utils::Borders;
use js_sys::Array;
use std::ops::{Deref, Neg};
use wasm_bindgen::prelude::*;

// if set too high, lanes won't be drawn
const INFINITY: f64 = 100_000.;

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
    borders: Vec<((f64, f64), (f64, f64))>,
    dash_line: Array,
}

#[wasm_bindgen]
impl Road {
    pub fn new(x: f64, width: f64, lane_count: i32) -> Self {
        crate::utils::set_panic_hook();
        let left = x - width / 2.;
        let right = x + width / 2.;

        let top = INFINITY.neg();
        let bottom = INFINITY;

        let borders = vec![
            ((left, top), (left, bottom)),
            ((right, top), (right, bottom)),
        ];

        let dash_line = Array::new();
        dash_line.push(&JsValue::from(20));
        dash_line.push(&JsValue::from(20));

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

    pub fn lane_center(&self, lane_index: i32) -> f64 {
        let lane_width = self.width / self.lane_count as f64;
        self.left + lane_width / 2. + (lane_index.min(self.lane_count - 1) as f64) * lane_width
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
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

        self.borders.iter().for_each(|(start, end)| {
            ctx.begin_path();
            ctx.move_to(start.0, start.1);
            ctx.line_to(end.0, end.1);
            ctx.stroke();
        });
    }
}

impl Road {
    pub fn boarders(&self) -> &Borders {
        self.borders.deref()
    }
}

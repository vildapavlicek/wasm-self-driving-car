use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::{car::Car, road::Road};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Traffic(#[wasm_bindgen(skip)] pub Vec<Car>);

#[wasm_bindgen]
impl Traffic {
    pub fn new() -> Self {
        Traffic(vec![])
    }

    pub fn add(&mut self, car: Car) {
        self.0.push(car);
    }

    pub fn add_car(&mut self, x: f64, y: f64, width: f64, height: f64, max_speed: f64) {
        self.0.push(Car::no_control(x, y, width, height, max_speed))
    }

    pub fn update(&mut self, road: &Road) {
        for car in &mut self.0 {
            car.update(road, &Traffic::new());
        }
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d, road: &Road) {
        for car in &mut self.0 {
            car.draw(ctx, road, &Traffic::new());
        }
    }
}

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

    pub fn with_capacity(capacity: usize) -> Self {
        Traffic(Vec::with_capacity(capacity))
    }

    pub fn add(&mut self, car: Car) {
        self.0.push(car);
    }

    pub fn add_random_car(&mut self, y: f64, road: &Road) {
        let at_lane = road.lane_center(js_sys::Math::floor(
            js_sys::Math::random() * road.lane_count() as f64,
        ) as i32);
        let speed = js_sys::Math::random() * 3.;

        crate::log!("adding car at lane {} with speed {}", at_lane, speed);

        let car = Car::no_control(at_lane, y - 500., speed);

        self.0.push(car);
    }

    pub fn add_car(&mut self, x: f64, y: f64, max_speed: f64) {
        self.0.push(Car::no_control(x, y, max_speed))
    }

    pub fn update(&mut self) {
        for car in &mut self.0 {
            car.update_dummy_car();
        }
    }

    pub fn draw(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        car_rendering_distance: f64,
        focused_agent_y: f64,
    ) {
        for car in &mut self.0 {
            // we take focused agent's y subtract it from car's y, this should give us the distance between those two
            // and if the distance is bigger than rendering distance, it means that car should be outside of visible canvas
            // so we shouldn't have a need to render it
            if (focused_agent_y.abs() - car.y.abs()).abs() > car_rendering_distance {
                continue;
            }
            car.draw(ctx, false);
        }
    }

    pub fn clean(&mut self, y: f64) {
        self.0.retain(|car| car.y.abs() > y.abs() - 500.);
    }
}

impl Default for Traffic {
    fn default() -> Self {
        Traffic::new()
    }
}

use crate::{
    controls::{ControlType, Controls, KeyEvent},
    road::Road,
    sensors::Sensor,
    traffic::Traffic,
};
use std::ops::Neg;
use std::{f64::consts::PI, ops::Deref};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const ANGLE_TURN: f64 = 0.03;
const FRICTION: f64 = 0.05;
const ACCELERATION: f64 = 0.2;

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Car {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
    max_speed: f64,
    angle: f64, // = 0;
    controls: Controls,
    sensor: Option<Sensor>,
    polygons: Vec<(f64, f64)>,
    damaged: bool,
}

#[wasm_bindgen]
impl Car {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        controls: Controls,
        max_speed: f64,
    ) -> Self {
        let sensor = match controls.control_type {
            ControlType::Keyboard => Some(Sensor::new(5, 100., std::f64::consts::PI / 2.)),
            _ => None,
        };

        Car {
            x,
            y,
            width,
            height,
            speed: 0.0,
            angle: 0.0,
            controls,
            sensor,
            polygons: vec![],
            damaged: false,
            max_speed,
        }
    }

    pub fn keyboard_controlled(x: f64, y: f64, width: f64, height: f64) -> Self {
        Car::new(
            x,
            y,
            width,
            height,
            Controls::new(ControlType::Keyboard),
            3.0,
        )
    }

    pub fn no_control(x: f64, y: f64, width: f64, height: f64, max_speed: f64) -> Self {
        Car::new(x, y, width, height, Controls::default(), max_speed)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn handle_key_input(&mut self, event: KeyEvent) {
        if let ControlType::Keyboard = self.controls.control_type {
            self.controls.handle_key_input(event);
        }
    }

    pub fn decelerate(&mut self) {
        self.speed -= 2.;
    }

    pub fn turn_left(&mut self) {
        self.angle += ANGLE_TURN;
    }

    pub fn turn_right(&mut self) {
        self.angle -= ANGLE_TURN;
    }

    pub fn update(&mut self, road: &Road, traffic: &Traffic) {
        match self.damaged {
            true => (),
            _ => {
                self.move_car();

                self.create_polygon();
                self.damaged = self.resolve_damage(road, traffic);
            }
        }

        if let Some(sensor) = self.sensor.as_mut() {
            sensor.update(self.x, self.y, self.angle, road.boarders(), traffic);
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, road: &Road, traffic: &Traffic) {
        match self.damaged {
            true => ctx.set_fill_style(&JsValue::from_str("gray")),
            false => ctx.set_fill_style(&JsValue::from_str("black")),
        };

        ctx.begin_path();
        let first = self.polygons.first().unwrap();
        ctx.move_to(first.0, first.1);

        self.polygons
            .iter()
            .skip(1)
            .for_each(|p| ctx.line_to(p.0, p.1));

        ctx.fill();

        if let Some(sensor) = self.sensor.as_ref() {
            sensor.draw(ctx, road.boarders(), traffic);
        }
    }
}

impl Car {
    fn move_car(&mut self) {
        if self.controls.up() {
            self.speed += ACCELERATION;
        }

        if self.controls.down() {
            self.speed -= 2.;
        }

        if self.speed != 0. {
            let flip = if self.speed > 0. { 1. } else { -1. };
            if self.controls.left() {
                self.angle += ANGLE_TURN * flip;
            }

            if self.controls.right() {
                self.angle -= ANGLE_TURN * flip;
            }
        }

        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }

        if self.speed < self.max_speed.neg() / 2. {
            self.speed = self.max_speed.neg() / 2.;
        }

        if self.speed > 0. {
            self.speed -= FRICTION;
        }

        if self.speed < 0. {
            self.speed += FRICTION;
        }

        if self.speed.abs() < FRICTION {
            self.speed = 0.;
        }

        self.x -= self.angle.sin() * self.speed;
        self.y -= self.angle.cos() * self.speed;
    }

    fn create_polygon(&mut self) {
        self.polygons.clear();
        let rad = self.width.hypot(self.height) / 2.;
        let alpha = self.width.atan2(self.height);

        // compute top right corner
        self.polygons.push((
            self.x - (self.angle - alpha).sin() * rad,
            self.y - (self.angle - alpha).cos() * rad,
        ));

        // compute top left corner
        self.polygons.push((
            self.x - (self.angle + alpha).sin() * rad,
            self.y - (self.angle + alpha).cos() * rad,
        ));

        // compute bottom right corner
        self.polygons.push((
            self.x - (PI + self.angle - alpha).sin() * rad,
            self.y - (PI + self.angle - alpha).cos() * rad,
        ));

        // compute bottom left corner
        self.polygons.push((
            self.x - (PI + self.angle + alpha).sin() * rad,
            self.y - (PI + self.angle + alpha).cos() * rad,
        ));
    }

    pub fn polygons(&self) -> &[(f64, f64)] {
        self.polygons.deref()
    }

    fn resolve_damage(&mut self, road: &Road, traffic: &Traffic) -> bool {
        if crate::utils::poly_intersection_with_borders(self.polygons.deref(), road.boarders()) {
            return true;
        };

        traffic
            .0
            .iter()
            .find(|car| crate::utils::poly_intersection_with_poly(self.polygons(), car.polygons()))
            .is_some()
    }
}

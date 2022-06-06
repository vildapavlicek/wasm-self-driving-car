use crate::{
    controls::{Controls, KeyEvent},
    road::Road,
    sensors::Sensor,
};
use std::ops::Neg;
use std::{f64::consts::PI, ops::Deref};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const ANGLE_TURN: f64 = 0.03;
const FRICTION: f64 = 0.05;
const MAX_SPEED: f64 = 3.;
const ACCELERATION: f64 = 0.2;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Car {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
    angle: f64, // = 0;
    controls: Controls,
    sensor: Sensor,
    polygons: Vec<(f64, f64)>,
    damaged: bool,
}

#[wasm_bindgen]
impl Car {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Car {
        Car {
            x,
            y,
            width,
            height,
            speed: 0.0,
            angle: 0.0,
            controls: Controls::default(),
            sensor: Sensor::new(3, 100., std::f64::consts::PI / 4.),
            polygons: vec![],
            damaged: false,
        }
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

    pub fn handle_key_input(&self, event: KeyEvent) {
        self.controls.handle_key_input(event);
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

    pub fn update(&mut self, road: &Road) {
        self.move_car();

        self.create_polygon();
        self.resolve_damage(road);

        self.sensor
            .update(self.x, self.y, self.angle, road.boarders());
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, road: &Road) {
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

        self.sensor.draw(ctx, road.boarders());
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

        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }

        if self.speed < MAX_SPEED.neg() / 2. {
            self.speed = MAX_SPEED.neg() / 2.;
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

    fn resolve_damage(&mut self, road: &Road) {
        self.damaged = crate::utils::polys_intersection(self.polygons.deref(), road.boarders());
    }
}

use crate::controls::{Controls, KeyEvent};
use wasm_bindgen::prelude::*;

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
    /*
    pub fn accelerate(&mut self) {
        self.speed += ACCELERATION;
        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }
    } */

    pub fn decelerate(&mut self) {
        self.speed -= 2.;
    }

    pub fn turn_left(&mut self) {
        self.angle += ANGLE_TURN;
    }

    pub fn turn_right(&mut self) {
        self.angle -= ANGLE_TURN;
    }

    pub fn update(&mut self) {
        use std::ops::Neg;

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
}

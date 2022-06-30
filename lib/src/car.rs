use crate::{
    ai::NeuralNetwork,
    controls::{ControlType, Controls},
    road::Road,
    sensors::Sensor,
    traffic::Traffic,
    Config,
};
use std::ops::Neg;
use std::{f64::consts::PI, ops::Deref};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const ANGLE_TURN: f64 = 0.03;
const FRICTION: f64 = 0.05;
const ACCELERATION: f64 = 0.2;

// const RAYS_COUNT: usize = 5;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Car {
    #[wasm_bindgen(skip)]
    pub id: usize,
    x: f64,
    pub y: f64,
    width: f64,
    height: f64,
    speed: f64,
    max_speed: f64,
    angle: f64, // = 0;
    controls: Controls,
    sensor: Option<Sensor>,
    brain: Option<NeuralNetwork>,
    polygons: Vec<(f64, f64)>,
    pub damaged: bool,
}

impl Car {
    pub fn no_control(x: f64, y: f64, max_speed: f64) -> Self {
        Car::new(
            0,
            x,
            y,
            30.,
            50.,
            Controls::default(),
            max_speed,
            None,
            None,
        )
    }

    pub fn with_brain(
        id: usize,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        sensor: Sensor,
        neurons_counts: &[usize],
        brain: Option<NeuralNetwork>,
    ) -> Self {
        Car::new(
            id,
            x,
            y,
            width,
            height,
            Controls::new(ControlType::Ai),
            3.0,
            Some(sensor),
            brain.or_else(|| Some(NeuralNetwork::new(neurons_counts))),
        )
    }

    pub fn set_brain(&mut self, brain: Option<NeuralNetwork>) {
        self.brain = brain;
    }

    /*     pub fn handle_key_input(&mut self, event: KeyEvent) {
           if let ControlType::Keyboard = self.controls.control_type {
               self.controls.handle_key_input(event);
           }
       }
    */

    pub fn mutate(&mut self, mutation: f64) {
        self.brain = self.brain.take().map(|brain| brain.mutate(mutation));
    }

    pub fn update(&mut self, road: &Road, traffic: &Traffic) {
        self.move_car();

        self.create_polygon();
        self.damaged = self.resolve_damage(road, traffic);

        if let Some(sensor) = self.sensor.as_mut() {
            sensor.update(self.x, self.y, self.angle, road.boarders(), traffic);

            if let Some(brain) = self.brain.as_mut() {
                let offsets = sensor
                    .readings()
                    .iter()
                    .map(|x| x.map(|i| 1. - i.offset).unwrap_or_default())
                    .collect::<Vec<f64>>();

                brain.feed_forward_2(offsets);
                let outputs = brain
                    .0
                    .last()
                    .expect("missing ouput layer")
                    .outputs
                    .borrow();

                self.controls.up = outputs[0] == 1.;
                self.controls.left = outputs[1] == 1.;
                self.controls.right = outputs[2] == 1.;
                self.controls.down = outputs[3] == 1.;
            }
        }
    }

    pub fn update_dummy_car(&mut self) {
        self.move_car();
        self.create_polygon();
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, draw_sensor: bool) {
        match (self.damaged, self.controls.control_type) {
            (true, _) => ctx.set_fill_style(&JsValue::from_str("gray")),
            (false, ControlType::Keyboard) => ctx.set_fill_style(&JsValue::from_str("blue")),
            (false, ControlType::NoControl) => ctx.set_fill_style(&JsValue::from_str("red")),
            _ => ctx.set_fill_style(&JsValue::from_str("cyan")),
        };

        ctx.begin_path();
        let first = self.polygons.first().unwrap();
        ctx.move_to(first.0, first.1);

        self.polygons
            .iter()
            .skip(1)
            .for_each(|p| ctx.line_to(p.0, p.1));

        ctx.fill();

        match self.sensor.as_ref() {
            Some(sensor) if draw_sensor => sensor.draw(ctx),
            _ => (),
        }
    }
}

impl Car {
    fn new(
        id: usize,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        controls: Controls,
        max_speed: f64,
        sensor: Option<Sensor>,
        brain: Option<NeuralNetwork>,
    ) -> Self {
        Car {
            id,
            x,
            y,
            width,
            height,
            speed: 0.0,
            angle: 0.0,
            controls,
            sensor,
            brain,
            polygons: vec![],
            damaged: false,
            max_speed,
        }
    }

    pub fn ai_default(id: usize, lane: f64, brain: Option<NeuralNetwork>, config: &Config) -> Self {
        Car::with_brain(
            id,
            lane,
            crate::CAR_Y_DEFAULT,
            crate::CAR_WIDHT_DEFAULT,
            crate::CAR_HEIGHT_DEFAULT,
            Sensor::new(
                config.rays_count as i32,
                config.rays_length,
                std::f64::consts::PI / config.rays_spread,
            ),
            &config.neurons_counts(),
            brain,
        )
    }

    pub fn brain(&self) -> Option<&NeuralNetwork> {
        self.brain.as_ref()
    }

    /// Generates vector of cars that will differ only in their brains
    /// # Arguments
    /// * `count` - number of cars to generate
    /// * `y` - y coordinate
    /// * `brain` - brain to use for each car, first car will have original brain, other brains will be mutated
    /// * `mutation_rate` - mutation rate for each brain expect first one
    pub fn generate_cars_same(x: f64, brain: Option<NeuralNetwork>, config: &Config) -> Vec<Car> {
        let mut cars_count = config.cars_count;
        let mut cars = Vec::with_capacity(config.cars_count);

        if brain.is_some() {
            cars.push(Car::ai_default(config.cars_count, x, brain.clone(), config));
            cars_count -= 1;
        }

        (0..cars_count).for_each(|n| {
            cars.push(Car::ai_default(
                n,
                x,
                brain.clone().map(|b| b.mutate(config.mutation_rate)),
                config,
            ))
        });

        cars
    }

    fn move_car(&mut self) {
        if self.controls.up() {
            self.speed += ACCELERATION;
        }

        if self.controls.down() {
            self.speed -= ACCELERATION;
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

        traffic.0.iter().any(|car| {
            match (self.y.abs() - car.y.abs()).abs() > 100. {
                // if car not in range, no damage can be done
                true => false,
                false => crate::utils::poly_intersection_with_poly(self.polygons(), car.polygons()),
            }
        })
    }
}

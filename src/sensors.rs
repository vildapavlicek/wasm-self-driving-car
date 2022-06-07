use crate::{
    traffic::Traffic,
    utils::{get_intersection, IntersectionPoint},
};
use itertools::Itertools;
use std::ops::Neg;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sensor {
    ray_count: i32,
    ray_length: f64,
    ray_spread: f64,
    //#[wasm_bindgen(skip)]
    /// [((start_x, start_y), (end_x, end_y))]
    rays: Vec<((f64, f64), (f64, f64))>,
    readings: Vec<Option<IntersectionPoint>>,
}

#[wasm_bindgen]
impl Sensor {
    pub fn new(ray_count: i32, ray_length: f64, ray_spread: f64) -> Self {
        let mut rays = Vec::with_capacity(ray_count as usize);
        for _ in 0..ray_count {
            rays.push(((0., 0.), (0., 0.)));
        }
        Sensor {
            ray_count,
            ray_length,
            ray_spread,
            rays,
            readings: vec![None; ray_count as usize],
        }
    }
}

impl Sensor {
    fn cast_rays(&mut self, x: f64, y: f64, angle: f64) {
        for (i, ray) in self.rays.iter_mut().enumerate() {
            let ray_angle = crate::utils::lerp(
                self.ray_spread / 2.,
                self.ray_spread.neg() / 2.,
                if self.ray_count == 1 {
                    0.5
                } else {
                    i as f64 / (self.ray_count - 1) as f64
                },
            ) + angle;

            let start = (x, y);
            let end = (
                x - ray_angle.sin() * self.ray_length,
                y - ray_angle.cos() * self.ray_length,
            );

            ray.0 = start;
            ray.1 = end;
        }
    }

    pub fn update(
        &mut self,
        x: f64,
        y: f64,
        angle: f64,
        road_borders: &[((f64, f64), (f64, f64))],
        traffic: &Traffic,
    ) {
        self.cast_rays(x, y, angle);
        self.rays
            .iter()
            .zip(self.readings.iter_mut())
            .for_each(|(ray, reading)| *reading = get_reading(*ray, road_borders, traffic));
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        for ((start, end), reading) in self.rays.iter().zip(self.readings.iter()) {
            let contact_point = match reading {
                Some(contact_point) => (contact_point.x, contact_point.y),
                None => *end,
            };

            ctx.begin_path();
            ctx.set_line_width(2.);
            ctx.set_stroke_style(&JsValue::from_str("yellow"));
            // from start of the ray until the contat point we will draw yellow line
            ctx.move_to(start.0, start.1);
            ctx.line_to(contact_point.0, contact_point.1);
            ctx.stroke();

            ctx.begin_path();
            ctx.set_line_width(2.);
            ctx.set_stroke_style(&JsValue::from_str("black"));
            // from the end of the ray we will draw black line until the contact point
            ctx.move_to(end.0, end.1);
            ctx.line_to(contact_point.0, contact_point.1);
            ctx.stroke();
        }
    }

    pub fn readings(&self) -> &[Option<IntersectionPoint>] {
        &self.readings
    }
}

fn get_reading(
    (ray_start, ray_end): ((f64, f64), (f64, f64)),
    road_borders: &[((f64, f64), (f64, f64))],
    traffic: &Traffic,
) -> Option<IntersectionPoint> {
    let mut contacts = vec![];
    road_borders.iter().for_each(|(border_start, border_end)| {
        get_intersection(ray_start, ray_end, *border_start, *border_end).and_then(
            |intersection_point| {
                contacts.push(intersection_point);
                Some(())
            },
        );
    });

    for car in traffic.0.iter() {
        let poly = car.polygons();
        for (poly_w_1, poly_w_2) in poly.iter().circular_tuple_windows() {
            get_intersection(ray_start, ray_end, *poly_w_1, *poly_w_2).and_then(
                |intersection_point| {
                    contacts.push(intersection_point);
                    Some(())
                },
            );
        }
    }

    if contacts.is_empty() {
        return None;
    }

    contacts
        .into_iter()
        .min_by(|x, y| x.offset.partial_cmp(&y.offset).unwrap())
}

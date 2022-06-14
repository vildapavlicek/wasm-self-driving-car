use crate::{
    traffic::Traffic,
    utils::{get_intersection, Borders, IntersectionPoint},
};
use itertools::Itertools;
use std::ops::Neg;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

type StartXY = (f64, f64);
type EndXY = (f64, f64);

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Sensor {
    ray_count: i32,
    ray_length: f64,
    ray_spread: f64,
    rays: Vec<(StartXY, EndXY)>,
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
        road_borders: &Borders,
        traffic: &Traffic,
    ) {
        self.cast_rays(x, y, angle);
        self.rays
            .iter()
            .zip(self.readings.iter_mut())
            .for_each(|(ray, reading)| *reading = get_reading(y, *ray, road_borders, traffic));
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
    reader_y: f64,
    (ray_start, ray_end): ((f64, f64), (f64, f64)),
    road_borders: &Borders,
    traffic: &Traffic,
) -> Option<IntersectionPoint> {
    let mut min_contact: Option<IntersectionPoint> = None;

    // let mut contacts = vec![];
    road_borders.iter().for_each(|(border_start, border_end)| {
        if let Some(intersection) = get_intersection(ray_start, ray_end, *border_start, *border_end)
        {
            match min_contact {
                Some(cp) if cp.offset > intersection.offset => min_contact = Some(intersection),
                None => min_contact = Some(intersection),
                _ => (),
            }
        }
    });

    for car in traffic.0.iter() {
        // let's skip cars that are out of sensor's range
        if (reader_y.abs() - car.y.abs()).abs() > 200. {
            continue;
        }
        let poly = car.polygons();
        for (poly_w_1, poly_w_2) in poly.iter().circular_tuple_windows() {
            if let Some(intersection) = get_intersection(ray_start, ray_end, *poly_w_1, *poly_w_2) {
                {
                    match min_contact {
                        Some(cp) if cp.offset > intersection.offset => {
                            min_contact = Some(intersection)
                        }
                        None => min_contact = Some(intersection),
                        _ => (),
                    }
                }
            }
        }
    }

    min_contact
}

pub mod ai;
pub mod car;
pub mod controls;
pub mod road;
pub mod sensors;
pub mod traffic;
pub mod utils;
pub mod visualizer;

use std::ops::Deref;

use ai::NeuralNetwork;
use car::Car;
use road::Road;
use traffic::Traffic;
use visualizer::Visualizer;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub const CARS_COUNT_DEFAULT: usize = 100;
pub const CAR_Y_DEFAULT: f64 = 100.;
pub const CAR_WIDHT_DEFAULT: f64 = 30.;
pub const CAR_HEIGHT_DEFAULT: f64 = 50.;
pub const RAYS_COUNT_DEFAULT: usize = 5;
pub const RAYS_LENGTH_DEFAULT: f64 = 120.;

pub const LANES_COUNT_DEFAULT: i32 = 3;

pub const NEURONS_COUNTS_DEFAULT: [usize; 3] = [RAYS_COUNT_DEFAULT, 6, 4];
pub const MUTATION_RATE_DEFAULT: f64 = 0.3;

const LOCAL_STORAGE_KEY: &str = "bestBrain";

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SimulationState {
    Running,
    Paused,
    Stopped,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Config {
    #[wasm_bindgen(js_name = lanesCount)]
    pub lanes_count: usize,
    #[wasm_bindgen(js_name = laneIndex)]
    pub lane_index: usize,
    #[wasm_bindgen(js_name = carsCount)]
    pub cars_count: usize,
    #[wasm_bindgen(js_name = raysCount)]
    pub rays_count: usize,
    #[wasm_bindgen(js_name = raysLength)]
    pub rays_lenght: f64,
    #[wasm_bindgen(skip)]
    pub hidden_layers: Vec<usize>,
    #[wasm_bindgen(js_name = mutationRate)]
    pub mutation_rate: f64,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(
        lanes_count: usize,
        lane_index: usize,
        cars_count: usize,
        rays_count: usize,
        rays_lenght: f64,
        hidden_layers: js_sys::Uint32Array,
        mutation_rate: f64,
    ) -> Self {
        Self {
            lanes_count,
            lane_index,
            cars_count,
            rays_count,
            rays_lenght,
            hidden_layers: hidden_layers
                .to_vec()
                .into_iter()
                .map(|x| x as usize)
                .collect(),
            mutation_rate,
        }
    }

    #[wasm_bindgen(method, getter = hiddenLayers)]
    pub fn hidden_layers(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(
            self.hidden_layers
                .iter()
                .map(|x| *x as u32)
                .collect::<Vec<u32>>()
                .deref(),
        )
    }

    #[wasm_bindgen(method, setter = hiddenLayers)]
    pub fn set_hidden_layers(&mut self, values: js_sys::Uint32Array) {
        self.hidden_layers = values.to_vec().into_iter().map(|x| x as usize).collect();
    }
}

impl Config {
    pub fn neurons_count(&self) -> Vec<usize> {
        let mut tmp = self.hidden_layers.to_vec();
        tmp.insert(0, self.rays_count);
        tmp.push(4);
        tmp
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Simulation {
    pub state: SimulationState,
    traffic: traffic::Traffic,
    cars: Vec<car::Car>,
    road: road::Road,
    config: Config,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn init(car_canvas_width: f64, window: &web_sys::Window, config: &Config) -> Self {
        let road = road::Road::new(
            car_canvas_width / 2.,
            car_canvas_width * 0.9,
            config.lanes_count as i32,
        );

        let brain = match window.local_storage() {
            Ok(Some(storage)) => match storage.get_item("bestBrain").ok().flatten() {
                Some(raw_brain) => {
                    log!("found stored brain");
                    NeuralNetwork::deserialize_brain(raw_brain)
                }
                _ => None,
            },
            _ => None,
        };

        let cars = Car::generate_cars_same(
            road.lane_center(config.lane_index as i32),
            brain.clone(),
            &config,
        );

        Simulation::new(road, cars, Traffic::new(), config.clone())
    }

    pub fn run(&mut self) {
        self.state = SimulationState::Running;
    }

    pub fn pause(&mut self) {
        self.state = SimulationState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = SimulationState::Stopped;
    }

    pub fn destroy(self) {
        drop(self);
    }

    pub fn step(
        &mut self,
        car_ctx: CanvasRenderingContext2d,
        network_ctx: CanvasRenderingContext2d,
    ) {
        self.update();
        self.draw(&car_ctx, &network_ctx);
    }

    #[wasm_bindgen(js_name = updateConfig)]
    pub fn update_config(&mut self, config: &Config) {
        self.config = config.clone();
    }

    pub fn add_basic_traffic(mut self) -> Self {
        self.traffic
            .add(Car::no_control(self.road.lane_center(0), -50., 2.));
        self.traffic
            .add(Car::no_control(self.road.lane_center(2), -50., 2.));
        //
        self.traffic
            .add(Car::no_control(self.road.lane_center(1), -150., 2.));
        //
        self.traffic
            .add(Car::no_control(self.road.lane_center(0), -250., 2.));
        self.traffic
            .add(Car::no_control(self.road.lane_center(1), -250., 2.));

        self
    }

    pub fn save_best_car(&self, window: &web_sys::Window) {
        let best_car = self
            .cars
            .iter()
            .min_by(|c1, c2| c1.y.partial_cmp(&c2.y).unwrap())
            .unwrap();

        let serialized_brain = best_car
            .brain()
            .expect("best car doesn't have brain")
            .serialize_brain();

        window
            .local_storage()
            .ok()
            .flatten()
            .expect("failed to get local storage")
            .set_item(LOCAL_STORAGE_KEY, serialized_brain.as_str())
            .expect("failed to save brain to local storage");
    }

    pub fn discard_brain(&self, window: &web_sys::Window) {
        window
            .local_storage()
            .ok()
            .flatten()
            .expect("failed to get local storage")
            .delete(LOCAL_STORAGE_KEY)
            .expect("failed to delete '{LOCAL_STORAGE_KEY}' from local storage");
    }
}

impl Simulation {
    fn new(
        road: Road,
        // brain: Option<NeuralNetwork>,
        cars: Vec<Car>,
        traffic: Traffic,
        config: Config,
    ) -> Self {
        Simulation {
            state: SimulationState::Stopped,
            traffic,
            cars,
            road,
            config,
        }
    }

    fn update(&mut self) {
        if !matches!(self.state, SimulationState::Running) {
            return;
        }

        // update traffic
        self.traffic.update(&self.road);

        // update cars
        for car in self.cars.iter_mut() {
            car.update(&self.road, &self.traffic);
        }
    }

    fn draw(&mut self, car_ctx: &CanvasRenderingContext2d, network_ctx: &CanvasRenderingContext2d) {
        if matches!(self.state, SimulationState::Stopped) {
            return;
        }
        // choose our best car
        let best_car = self
            .cars
            .iter_mut()
            .min_by(|c1, c2| c1.y.partial_cmp(&c2.y).unwrap())
            .unwrap();

        // draw best cars neural network
        network_ctx.set_line_dash_offset(best_car.y / 5.);
        Visualizer::draw_network(
            &network_ctx,
            best_car.brain().expect("best car doesn't have brain"),
        );

        // save context
        car_ctx.save();
        // move canvas
        car_ctx
            .translate(
                0.,
                -best_car.y + car_ctx.canvas().unwrap().height() as f64 * 0.7,
            )
            .expect("failed to translate on saved context");
        self.road.draw(&car_ctx);
        self.traffic.draw(&car_ctx);

        // first draw best car so we can drop it later
        best_car.draw(&car_ctx, true);

        // draw rest of the cars
        car_ctx.set_global_alpha(0.2);
        for car in self.cars.iter() {
            car.draw(&car_ctx, false);
        }
        car_ctx.set_global_alpha(1.);

        car_ctx.restore();
    }
}

pub mod ai;
pub mod car;
pub mod controls;
pub mod road;
pub mod sensors;
pub mod traffic;
pub mod utils;
pub mod visualizer;

use std::ops::Deref;

use ai::{agents::Agents, NeuralNetwork};
use car::Car;
use js_sys::Uint32Array;
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

#[macro_export]
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
    }
}

pub const CAR_Y_DEFAULT: f64 = 100.;
pub const CAR_WIDHT_DEFAULT: f64 = 30.;
pub const CAR_HEIGHT_DEFAULT: f64 = 50.;

const LOCAL_STORAGE_KEY: &str = "bestBrain";

const IDEAL_DISTANCE: f64 = -250.;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Save {
    brain: NeuralNetwork,
    config: Config,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SimulationState {
    Running,
    Paused,
    Stopped,
}

#[wasm_bindgen]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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
    #[wasm_bindgen(js_name = raysSpread)]
    pub rays_spread: f64,
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
        rays_spread: f64,
        hidden_layers: js_sys::Uint32Array,
        mutation_rate: f64,
    ) -> Self {
        Self {
            lanes_count,
            lane_index,
            cars_count,
            rays_count,
            rays_lenght,
            rays_spread,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            lanes_count: 3,
            lane_index: 1,
            cars_count: 100,
            rays_count: 5,
            rays_lenght: 120.,
            rays_spread: 2.,
            hidden_layers: vec![6],
            mutation_rate: 0.2,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Simulation {
    pub state: SimulationState,
    traffic: traffic::Traffic,
    agents: Agents,
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
                Some(raw_save) => {
                    log!("found stored brain");
                    Some(
                        serde_json::from_str::<Save>(raw_save.as_str())
                            .map(|s| s.brain)
                            .expect("failed to deserialize save data"),
                    )
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

        Simulation::new(road, Agents::new(cars), Traffic::new(), config.clone())
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

    pub fn next_agent(&mut self) {
        self.agents.focus_next();
    }

    pub fn previous_agent(&mut self) {
        self.agents.focus_previous();
    }

    #[wasm_bindgen(js_name = spawnCar)]
    pub fn spawn_car(&mut self, lane_index: i32) {
        self.traffic.add_car(
            self.road.lane_center(lane_index),
            self.agents
                .best_agent()
                .expect("no best agent, can't resolve Y coordinate")
                .y
                - 500.,
            2.,
        )
    }

    #[wasm_bindgen(js_name = spawnRandom)]
    pub fn spawn_random(&mut self) {
        self.traffic.add_car(
            self.road.lane_center(
                (js_sys::Math::random() * (self.config.lanes_count + 1) as f64).floor() as i32,
            ),
            self.agents
                .best_agent()
                .expect("no best agent, can't resolve Y coordinate")
                .y
                - 500.,
            2.,
        )
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

    #[wasm_bindgen(js_name = top10Agents)]
    pub fn top_10_agents(&self) -> Uint32Array {
        Uint32Array::from(
            self.agents
                .n_best(10)
                .into_iter()
                .map(|(id, _)| id as u32)
                .collect::<Vec<u32>>()
                .deref(),
        )
    }

    #[wasm_bindgen(js_name = focusAgent)]
    pub fn focus_agent(&mut self, agent_id: usize) {
        self.agents.focus_agent(agent_id);
    }

    #[wasm_bindgen(js_name = resetFocus)]
    pub fn reset_focus(&mut self) {
        self.agents.focus_best_agent();
    }

    pub fn add_basic_traffic(mut self) -> Self {
        // |x| |x|
        // | | | |
        // |X|x| |
        // | | | |
        // | |x|x|
        // | | | |
        // |x|x| |
        // | | | |
        // | |x| |
        // | | | |
        // |x| |x|

        self.traffic
            .add(Car::no_control(self.road.lane_center(0), 0., 2.));
        self.traffic
            .add(Car::no_control(self.road.lane_center(2), 0., 2.));
        //
        //self.traffic
        //    .add(Car::no_control(self.road.lane_center(1), -150., 2.));
        //
        self.traffic.add(Car::no_control(
            self.road.lane_center(0),
            1_f64 * IDEAL_DISTANCE,
            2.,
        ));
        self.traffic.add(Car::no_control(
            self.road.lane_center(1),
            1_f64 * IDEAL_DISTANCE,
            2.,
        ));
        //
        self.traffic.add(Car::no_control(
            self.road.lane_center(1),
            2_f64 * IDEAL_DISTANCE,
            2.,
        ));
        self.traffic.add(Car::no_control(
            self.road.lane_center(2),
            2_f64 * IDEAL_DISTANCE,
            2.,
        ));
        //
        self.traffic.add(Car::no_control(
            self.road.lane_center(0),
            3_f64 * IDEAL_DISTANCE,
            2.,
        ));
        self.traffic.add(Car::no_control(
            self.road.lane_center(1),
            3_f64 * IDEAL_DISTANCE,
            2.,
        ));
        //
        self.traffic.add(Car::no_control(
            self.road.lane_center(0),
            4_f64 * IDEAL_DISTANCE,
            2.,
        ));
        self.traffic.add(Car::no_control(
            self.road.lane_center(2),
            4_f64 * IDEAL_DISTANCE,
            2.,
        ));
        self
    }

    #[wasm_bindgen(js_name = saveFocusedCar)]
    pub fn save_best_focused_car(&self, window: &web_sys::Window) {
        let save = Save {
            brain: self
                .agents
                .focused_agent()
                .expect("no best agent found")
                .brain()
                .expect("agent without brain")
                .clone(),
            config: self.config.clone(),
        };

        let serialized_data = serde_json::to_string(&save).expect("failed to serialize save data");

        window
            .local_storage()
            .ok()
            .flatten()
            .expect("failed to get local storage")
            .set_item(LOCAL_STORAGE_KEY, serialized_data.as_str())
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

    #[wasm_bindgen(js_name = initConfig)]
    pub fn init_config(window: web_sys::Window) -> Config {
        match window
            .local_storage()
            .ok()
            .flatten()
            .expect("failed to get local storage")
            .get_item(LOCAL_STORAGE_KEY)
            .expect("failed to retrieve storage data")
        {
            Some(item) => serde_json::from_str::<Save>(item.as_str())
                .map(|save| save.config)
                .unwrap_or_else(|_| Config::default()),
            None => Config::default(),
        }
    }
}

impl Simulation {
    fn new(road: Road, agents: Agents, traffic: Traffic, config: Config) -> Self {
        Simulation {
            state: SimulationState::Stopped,
            traffic,
            agents,
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
        self.agents.update(&self.road, &self.traffic)
    }

    fn draw(&mut self, car_ctx: &CanvasRenderingContext2d, network_ctx: &CanvasRenderingContext2d) {
        if matches!(self.state, SimulationState::Stopped) {
            return;
        }

        let focused_agent = self
            .agents
            .focused_agent()
            .expect("no focused agent, no agent to follow");

        // draw best cars neural network
        network_ctx.set_line_dash_offset(focused_agent.y / 5.);
        Visualizer::draw_network(
            &network_ctx,
            focused_agent
                .brain()
                .expect("best agent doesn't have brain"),
        );

        // save context
        car_ctx.save();
        // move canvas
        car_ctx
            .translate(
                0.,
                -focused_agent.y + car_ctx.canvas().unwrap().height() as f64 * 0.7,
            )
            .expect("failed to translate on saved context");
        self.road.draw(car_ctx);
        self.traffic.draw(car_ctx);

        self.agents.draw(car_ctx);

        car_ctx.restore();
    }
}

pub mod ai;
pub mod car;
mod config;
pub mod controls;
pub mod road;
pub mod sensors;
pub mod traffic;
pub mod utils;
pub mod visualizer;
pub use config::Config;
use tap::TapOptional;

use std::ops::Deref;

use ai::{agents::Agents, NeuralNetwork};
use car::Car;
use js_sys::Uint32Array;
use road::Road;
use traffic::Traffic;
use visualizer::Visualizer;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const CAR_Y_DEFAULT: f64 = 100.;
pub const CAR_WIDHT_DEFAULT: f64 = 30.;
pub const CAR_HEIGHT_DEFAULT: f64 = 50.;

const IDEAL_DISTANCE: f64 = -250.;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SimulationState {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Simulation {
    pub state: SimulationState,
    traffic: traffic::Traffic,
    agents: Agents,
    road: road::Road,
    config: Config,
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

    pub fn init(config: &Config) -> Self {
        let road = road::Road::new(200. / 2., 200. * 0.9, config.lanes_count as i32);
        let brain = ai::NeuralNetwork::load_brain();

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

    pub fn spawn_cars_vertically(&mut self, lane_indexes: Uint32Array) {
        for (i, lane_index) in lane_indexes.to_vec().into_iter().enumerate() {
            if !(0..self.config.lanes_count).contains(&(lane_index as usize)) {
                /*  error!(
                    "lane index {lane_index} out of range, mix 0, max {}",
                    self.config.lanes_count
                ); */
                continue;
            }

            self.traffic.add_car(
                self.road.lane_center(lane_index as i32),
                self.agents
                    .best_agent()
                    .expect("no best agent, can't resolve Y coordinate")
                    .y
                    + (-500. + (i as f64 * IDEAL_DISTANCE)),
                2.,
            )
        }
    }

    pub fn spawn_cars_horizontally(&mut self, lane_indexes: Uint32Array) {
        if lane_indexes.length() > self.config.lanes_count as u32 {
            // error!("number of lanes is bigger than actual lanes count");
            return;
        }

        for lane_index in lane_indexes.to_vec().into_iter() {
            self.traffic.add_car(
                self.road.lane_center(lane_index as i32),
                self.agents
                    .best_agent()
                    .expect("no best agent, can't resolve Y coordinate")
                    .y
                    - 500.,
                2.,
            )
        }
    }

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
        car_ctx: &CanvasRenderingContext2d,
        network_ctx: &CanvasRenderingContext2d,
        car_rendering_distance: f64,
    ) {
        self.update();
        self.draw(car_ctx, network_ctx, car_rendering_distance);
    }

    pub fn update_config(&mut self, config: &Config) {
        self.config = config.clone();
    }

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

    pub fn focus_agent(&mut self, agent_id: usize) {
        self.agents.focus_agent(agent_id);
    }

    pub fn reset_focus(&mut self) {
        self.agents.focus_best_agent();
    }

    pub fn add_basic_traffic(&mut self, distance_ratio: f64) {
        // | |x|x|
        // | | | |
        // |x| |x|
        // | | | |
        // |x|x| |
        // | | | |
        // |x| |X|
        // | | | |
        // | |x| |
        // | | | |
        // |x| |x|

        enum Test {
            One(i32),
            Two((i32, i32)),
        }

        let tests = vec![
            Test::Two((0, 2)),
            Test::Two((0, 2)),
            Test::One(1),
            Test::Two((0, 2)),
            Test::Two((0, 1)),
            Test::Two((0, 2)),
            Test::Two((1, 2)),
        ];

        let y = self
            .agents
            .best_agent()
            .expect("no best agent, can't resolve Y coordinate")
            .y;

        for (index, test) in tests.into_iter().enumerate() {
            match test {
                Test::One(lane) => {
                    self.traffic.add(Car::no_control(
                        self.road.lane_center(lane),
                        y + ((index + 1) as f64 * IDEAL_DISTANCE * distance_ratio),
                        2.,
                    ));
                }
                Test::Two((lane_1, lane_2)) => {
                    self.traffic.add(Car::no_control(
                        self.road.lane_center(lane_1),
                        y + ((index + 1) as f64 * IDEAL_DISTANCE * distance_ratio),
                        2.,
                    ));
                    self.traffic.add(Car::no_control(
                        self.road.lane_center(lane_2),
                        y + ((index + 1) as f64 * IDEAL_DISTANCE * distance_ratio),
                        2.,
                    ));
                }
            }
        }
    }

    pub fn training_traffic(&mut self) {
        const DISTANCE: f64 = 250.;

        let mut y = self
            .agents
            .best_agent()
            .expect("no best agent, can't resolve Y coordinate")
            .y
            .abs();

        y = y + 150.;

        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        // spread, avoid car in the middle, ie can turn left or right
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        // from spreading return to middle
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        // turn to right most lane
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        // spread
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        // return to middle
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        // turn to left most lane
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(2), -y, 2.));
        }

        // turn to right most lane
        y = y + DISTANCE;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }

        y = y + DISTANCE / 3.;
        for _ in 0..3 {
            y = y + CAR_HEIGHT_DEFAULT + 10.;
            self.traffic
                .add(Car::no_control(self.road.lane_center(0), -y, 2.));
            self.traffic
                .add(Car::no_control(self.road.lane_center(1), -y, 2.));
        }
    }

    pub fn save_best_focused_car(&self) {
        self.agents
            .best_agent()
            .tap_none(|| tracing::error!("best agent not found!"))
            .map(|agent| agent.brain())
            .flatten()
            .tap_some(|brain| brain.save_brain());
    }

    pub fn discard_brain(&self) {
        NeuralNetwork::discard_saved_brain()
    }

    pub fn focus_agent_y(&self) -> f64 {
        self.agents.best_agent().map(|c| c.y).unwrap_or_default()
    }

    fn update(&mut self) {
        if !matches!(self.state, SimulationState::Running) {
            return;
        }

        if let Some(a) = self.agents.best_agent() {
            self.traffic.clean(a.y);
        };

        // update traffic
        self.traffic.update();
        self.agents.clean();
        self.agents.update(&self.road, &self.traffic);
    }

    fn draw(
        &mut self,
        car_ctx: &CanvasRenderingContext2d,
        network_ctx: &CanvasRenderingContext2d,
        car_rendering_distance: f64,
    ) {
        if matches!(self.state, SimulationState::Stopped) {
            return;
        }

        let focused_agent = match self.agents.focused_agent() {
            Some(fa) => fa,
            None => {
                // error!("didn't find any focused agent. Agents\n {:#?}", self.agents);
                return;
            }
        };

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
        self.traffic
            .draw(car_ctx, car_rendering_distance, focused_agent.y);

        self.agents.draw(car_ctx);
        car_ctx.restore();
    }
}

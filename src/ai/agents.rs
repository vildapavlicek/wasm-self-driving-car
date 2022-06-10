use crate::{car::Car, road::Road, Traffic};
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
pub struct Agents {
    best_agent_index: usize,
    agents: Vec<Car>,
    scores: Vec<f64>,
}

impl Agents {
    pub fn new(cars: Vec<Car>) -> Self {
        Agents {
            best_agent_index: 0,
            scores: vec![0.0; cars.len()],
            agents: cars,
        }
    }

    pub fn best_agent(&self) -> Option<&Car> {
        self.agents.get(self.best_agent_index)
    }

    pub fn update(&mut self, road: &Road, traffic: &Traffic) {
        let mut tmp_score = 0.0;
        let mut tmp_index = 0_usize;
        for (i, car) in self.agents.iter_mut().enumerate() {
            car.update(road, traffic);

            let score = car.y.abs();
            if score > tmp_score {
                tmp_score = score;
                tmp_index = i;
            }

            self.scores[i] = score;
        }

        self.best_agent_index = tmp_index;
    }

    pub fn draw(&mut self, car_ctx: &CanvasRenderingContext2d) {
        car_ctx.set_global_alpha(0.2);
        for (i, car) in self.agents.iter().enumerate() {
            if i == self.best_agent_index {
                continue;
            }
            car.draw(car_ctx, false);
        }
        car_ctx.set_global_alpha(1.);

        self.agents
            .get_mut(self.best_agent_index)
            .unwrap()
            .draw(car_ctx, true);
    }
}

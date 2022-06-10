use crate::{car::Car, road::Road, Traffic};
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
enum Focus {
    /// Follow best agent
    BestAgent,
    /// Follow random agent no matter the score
    SpecificAgent(usize),
    // /// Follow specific agent based on score, for example first, second, third etc.
    // Offset { offset: usize },
}

impl Default for Focus {
    fn default() -> Self {
        Focus::BestAgent
    }
}

#[derive(Debug)]
pub struct Agents {
    best_agent_index: usize,
    agents: Vec<Car>,
    scores: Vec<f64>,
    focused_agent: Focus,
}

impl Agents {
    pub fn new(cars: Vec<Car>) -> Self {
        Agents {
            best_agent_index: 0,
            scores: vec![0.0; cars.len()],
            agents: cars,
            focused_agent: Focus::default(),
        }
    }

    pub fn best_agent(&self) -> Option<&Car> {
        self.agents.get(self.best_agent_index)
    }

    pub fn focused_agent(&self) -> Option<&Car> {
        match self.focused_agent {
            Focus::BestAgent => self.best_agent(),
            Focus::SpecificAgent(index) => self.agents.get(index),
        }
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
        let focus_agent_id = self.focus_agent_id();
        car_ctx.set_global_alpha(0.2);
        for (i, car) in self.agents.iter().enumerate() {
            if i == focus_agent_id {
                continue;
            }
            car.draw(car_ctx, false);
        }
        car_ctx.set_global_alpha(1.);

        self.agents
            .get_mut(focus_agent_id)
            .unwrap()
            .draw(car_ctx, true);
    }

    fn focus_agent_id(&self) -> usize {
        match self.focused_agent {
            Focus::BestAgent => self.best_agent_index,
            Focus::SpecificAgent(index) => index,
        }
    }

    pub fn focus_agent(&mut self, agent_id: usize) {
        self.focused_agent = Focus::SpecificAgent(agent_id);
    }

    pub fn focus_next(&mut self) {
        match self.focused_agent {
            Focus::BestAgent => {
                self.focused_agent =
                    Focus::SpecificAgent((self.best_agent_index + 1) % self.agents.len());
            }
            Focus::SpecificAgent(index) => {
                self.focused_agent = Focus::SpecificAgent((index + 1) % self.agents.len());
            }
        }
    }
    pub fn focus_previous(&mut self) {
        match self.focused_agent {
            Focus::BestAgent => {
                self.focused_agent = Focus::SpecificAgent(
                    (self.best_agent_index - 1).clamp(0, self.agents.len() - 1),
                );
            }
            Focus::SpecificAgent(index) => {
                self.focused_agent =
                    Focus::SpecificAgent((index - 1).clamp(0, self.agents.len() - 1))
            }
        }
    }

    pub fn second_best(&self) -> Option<&Car> {
        unimplemented!()
    }

    pub fn third_best(&self) -> Option<&Car> {
        unimplemented!()
    }
}

use std::{collections::HashMap, ops::Neg};

use crate::{car::Car, error, road::Road, Traffic};
use web_sys::CanvasRenderingContext2d;

type AgentId = usize;

#[derive(Debug)]
enum Focus {
    /// Follow best agent
    BestAgent,
    /// Follow set agent no matter the score
    /// Wrapped value is index of agent's position in `agents`
    SpecificAgent(usize),
}

impl Default for Focus {
    fn default() -> Self {
        Focus::BestAgent
    }
}

#[derive(Debug)]
pub struct Agents {
    /// Best agent should be most desired, so we will cache its index
    best_agent_index: usize,
    /// All of our agents we will operate on
    agents: Vec<Car>,
    /// Map that tracks scores of each agent
    /// `AgentId` is equivalent to `agent.id`
    scores: HashMap<AgentId, f64>,
    /// Decides on which agent to center our animation, visualize brain, store brain, show sensors and draw with full colors (not transparent)
    focused_agent: Focus,
}

impl Agents {
    pub fn new(cars: Vec<Car>) -> Self {
        Agents {
            best_agent_index: 0,
            scores: HashMap::new(),
            agents: cars,
            focused_agent: Focus::default(),
        }
    }

    /// Returns reference to agent with highest score
    pub fn best_agent(&self) -> Option<&Car> {
        self.agents.get(self.best_agent_index)
    }

    /// Returns reference to agent focused agent
    pub fn focused_agent(&self) -> Option<&Car> {
        match self.focused_agent {
            Focus::BestAgent => self.best_agent(),
            Focus::SpecificAgent(index) => self.agents.get(index),
        }
    }

    /// update all our agent related data such as score, position, etc.
    pub fn update(&mut self, road: &Road, traffic: &Traffic) {
        let mut tmp_score = 0.0;
        let mut tmp_index = 0_usize;
        for (i, car) in self.agents.iter_mut().enumerate() {
            car.update(road, traffic);

            // we start at y position of Y
            // so the cars that start going backwards are considered as highest scores
            // so we reverse their score
            let score = car.y.neg();
            if score > tmp_score {
                tmp_score = score;
                tmp_index = i;
            }

            let v = self.scores.entry(car.id).or_insert(0.);
            *v = score;
        }

        self.best_agent_index = tmp_index;
    }

    /// Draw all our agents on provided canvas
    pub fn draw(&mut self, car_ctx: &CanvasRenderingContext2d) {
        let focus_agent_index = self.focus_agent_index();
        car_ctx.set_global_alpha(0.2);
        for (i, car) in self.agents.iter().enumerate() {
            if i == focus_agent_index {
                continue;
            }
            car.draw(car_ctx, false);
        }
        car_ctx.set_global_alpha(1.);

        self.agents
            .get_mut(focus_agent_index)
            .unwrap()
            .draw(car_ctx, true);
    }

    /// helper to get index of the focused agent
    fn focus_agent_index(&self) -> usize {
        match self.focused_agent {
            Focus::BestAgent => self.best_agent_index,
            Focus::SpecificAgent(index) => index,
        }
    }

    /// Focuses agent with specific ID
    ///
    /// # Arguments
    /// * `agent_id` - ID of the agent to focus, used to find the index of the focused agent
    pub fn focus_agent(&mut self, agent_id: usize) {
        let index = self.agents.iter().position(|c| c.id == agent_id);

        match index {
            Some(index) => self.focused_agent = Focus::SpecificAgent(index),
            None => error!("Invalid agent id '{agent_id}'"),
        }
    }

    /// Changes which agent is rendered as focused (ie is not transparent, shows sensors' rays and visualizes agent's brain)
    /// Focuses next agent in the list
    /// It wraps around, so if current agent is last, then next will be first
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

    /// Changes which agent is rendered as focused (ie is not transparent, shows sensors' rays and visualizes agent's brain)
    /// It wraps around, so if current agent is first, then previous will be last
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

    pub fn focus_best_agent(&mut self) {
        self.focused_agent = Focus::BestAgent;
    }

    /// Get reference to the n-th agent
    ///
    /// # Arguments
    /// * `index` - index of the agent, ie best agent = 0, second best agent = 1, etc.
    ///
    /// # Returns
    /// * `Option<&Car>` - reference to the n-th best agent
    ///
    /// # Panic
    /// This function will panic if the index is out of bounds.
    pub fn nth_best(&self, index: usize) -> Option<&Car> {
        // we take our scores map and turn it into vector of pairs (agent_id, score)
        let mut data = self
            .scores
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<(usize, f64)>>();

        // sort our vector by score
        data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // now we find n-th best agent
        self.agents.iter().find(|car| {
            car.id
                == *data
                    // get n-th best (agent id, score) from our ordered vector, ie if n = 1, we should get agent with highest score
                    // if n = 2, we should get agent with second highest score etc.
                    .get(index)
                    .map(|(id, _)| id)
                    .expect("nth_best: n out of bounds")
        })
    }

    pub fn n_best(&self, count: usize) -> Vec<(AgentId, f64)> {
        let mut data = self
            .scores
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<(usize, f64)>>();

        data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        data.into_iter().take(count).collect()
    }
}

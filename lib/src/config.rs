use tap::prelude::*;

const LOCAL_STORAGE_CONFIG_KEY: &str = "wasm-self-driving-car.config.save";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Config {
    pub lanes_count: usize,
    pub lane_index: usize,
    pub cars_count: usize,
    pub rays_count: usize,
    pub rays_length: f64,
    pub rays_spread: f64,
    pub hidden_layers: Vec<usize>,
    pub mutation_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lanes_count: 3,
            lane_index: 1,
            cars_count: 100,
            rays_count: 5,
            rays_length: 120.,
            rays_spread: 2.,
            /// this value are only hidden layers, if you need to pass neuron conuts to construct neural network
            /// use [Config::neurons_counts] instead which adds input and output layer
            hidden_layers: vec![6],
            mutation_rate: 0.2,
        }
    }
}

impl Config {
    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to usize
    pub fn set_lanes_count(&mut self, value: usize) {
        self.lanes_count = value
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to usize
    pub fn set_lane_index(&mut self, value: usize) {
        self.lane_index = value
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to usize
    pub fn set_cars_count(&mut self, value: usize) {
        self.cars_count = value;
        tracing::trace!(count = self.cars_count, "cars count");
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to usize
    pub fn set_rays_count(&mut self, value: usize) {
        self.rays_count = value
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to f64
    pub fn set_rays_length(&mut self, value: f64) {
        self.rays_length = value
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * value - parsed to f64
    pub fn set_rays_spread(&mut self, value: f64) {
        self.rays_spread = value
    }

    /// Setter for [Config.hidden_layers]
    /// # Arguments
    /// * values - list of values separated by comma, example "1,2,3", parsed as Vec<usize>
    pub fn parse_and_set_hidden_layers(&mut self, values: &str) {
        let hidden_layers = values
            .split(',')
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        if hidden_layers.is_empty() {
            tracing::error!("failed to parse hidden layers");
            return;
        }

        self.hidden_layers = hidden_layers;
    }

    /// Sets mutation rate to privded value
    /// # Arguments
    /// * value - parsed to f64
    pub fn set_mutation_rate(&mut self, value: f64) {
        self.mutation_rate = value
    }

    /// Tries to load Config from web browser's local storage
    /// If it fails, returns [Config::default]
    pub fn load() -> Self {
        use gloo_storage::Storage;

        gloo_storage::LocalStorage::get(LOCAL_STORAGE_CONFIG_KEY)
            .tap(|config| tracing::info!(?config, "local storage config"))
            .unwrap_or_default()
    }

    /// Saves config to local storage of the web browser
    pub fn save(&self) {
        use gloo_storage::Storage;

        if let Err(err) = gloo_storage::LocalStorage::set(LOCAL_STORAGE_CONFIG_KEY, self) {
            tracing::error!(config = ?self, %err, "failed to save config to local storage");
        }
    }

    /// Returns hidden layers as a list of value separeted by comma as a String
    pub fn hidden_layers(&self) -> String {
        use std::io::Write;
        let mut buffer = Vec::with_capacity(self.hidden_layers.capacity());
        self.hidden_layers.iter().for_each(|n| {
            write!(&mut buffer, "{},", n).ok();
        });
        buffer.pop();

        String::from_utf8(buffer).expect("failed to write hidden layers as string")
    }

    /// Retuns count of neurons for each layer, including input, hidden and output layer
    /// where input layer = rays count and output is always 4 neurons
    pub(crate) fn neurons_counts(&self) -> Vec<usize> {
        let mut tmp = self.hidden_layers.to_vec();
        tmp.insert(0, self.rays_count);
        tmp.push(4);
        tmp
    }
}
